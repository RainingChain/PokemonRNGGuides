use super::{calc_modulo_cycle_u,calc_modulo_cycle_s};
use super::{Gen3WOpts, GeneratedPokemon};
use crate::Ivs;
use crate::gen3::EncounterSlot;
use crate::gen3::Gen3Lead;
use crate::gen3::Gen3Method;
use crate::rng::Rng;
use crate::rng::StateIterator;
use crate::rng::lcrng::Pokerng;
use crate::{AbilityType, Gender, GenderRatio, Nature, PkmFilter, ShinyType, gen3_shiny};

pub struct GeneratedPokemonCycle {
    pub cycle_start:usize,
    pub cycle_end:usize,
    pub gen_mon:GeneratedPokemon,
}

pub struct GeneratedPokemons {
    pub method1: Option<GeneratedPokemonCycle>,
    pub method2: Option<GeneratedPokemonCycle>,
    pub method3: Vec<GeneratedPokemonCycle>,
    pub method4: Option<GeneratedPokemonCycle>,
    pub method5: Vec<GeneratedPokemonCycle>,
    pub pid_reroll_cycle:usize,
    pub pid_reroll_count:usize,
}

impl GeneratedPokemons {
    fn add_method5(&mut self, gen_mon_cycle: GeneratedPokemonCycle) {
        if !self.method5.is_empty() {
            if self.method5.last().unwrap().cycle_end == gen_mon_cycle.cycle_start &&
               self.method5.last().unwrap().gen_mon.pid == gen_mon_cycle.gen_mon.pid &&
               self.method5.last().unwrap().gen_mon.ivs == gen_mon_cycle.gen_mon.ivs {
                self.method5.last_mut().unwrap().cycle_end = gen_mon_cycle.cycle_end;
                return;
            }
        }
        self.method5.push(gen_mon_cycle);
    }

    #[allow(dead_code)]
    fn new() -> Self {
        GeneratedPokemons {
            method1: None,
            method2: None,
            method3: Vec::new(),
            method4: None,
            method5: Vec::new(),
            pid_reroll_cycle:0,
            pid_reroll_count:0,
        }
    }
}

impl std::fmt::Debug for GeneratedPokemonCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PID: {}, IVS: {:?}, Cycle: {}-{}", 
            self.gen_mon.pid, self.gen_mon.ivs, self.cycle_start, self.cycle_end)
    }
}

impl std::fmt::Debug for GeneratedPokemons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[\n  Wild-1: {:?},\n  Wild-2: {:?},\n  Wild-3: {:?},\n  Wild-4: {:?},  Wild-5: {:?},\n\n]", 
            self.method1, self.method2, self.method3, self.method4, self.method5)
    }
}

const PID:u32 = 0;
const VERBOSE:bool = true;

// assumes synchronize in ability slot 0
// assume no hustle, vital srpiit, pressure, keen_eye, intimidate.

//NO_PROD only used cycle at Random() start. but need to consider exact operation where it matters.


#[allow(dead_code)]
pub fn generate_pokemons(rng: &mut Pokerng, settings: &Gen3WOpts) -> GeneratedPokemons {
    let mut cycle: usize = 74470; //NO_PROD

    let mut gen_mons = GeneratedPokemons::new();

    let encounter_rand_val = rng.rand::<u16>() as u32;
    let encounter_rand = (encounter_rand_val % 100) as u8;    
    cycle += calc_modulo_cycle_u(encounter_rand_val, 100);
    
    let encounter_slot = EncounterSlot::from_rand(encounter_rand);
    cycle += 378; //between ChooseWildMonIndex_Land and ChooseWildMonLevel

    if !EncounterSlot::passes_filter(settings.encounter_slot.as_deref(), encounter_slot) {
        return gen_mons;
    }

    let lvl_range_rand_val = rng.rand::<u16>(); // level
    let lvl_range = 1;
    cycle += calc_modulo_cycle_s(lvl_range_rand_val as i32, lvl_range);  
    
    //between ChooseWildMonLevel and PickWildMonNature_v1
    cycle += 48 * calc_modulo_cycle_u(PID, 24); 
    cycle += 30921; // range is about 30921-30939 for same pkm
    
    let nature_rand: u8;

    match settings.synchronize {
        None => {
            let nature_rand_val = rng.rand::<u16>();
            nature_rand = (nature_rand_val % 25) as u8;
            cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25); 
        }
        Some(Gen3Lead::Synchronize(lead_nature)) => {
            if (rng.rand::<u16>() & 1) == 0 {
                nature_rand = lead_nature.into();
                if VERBOSE { println!("Synchronize: {:?}", nature_rand); }
                //NO_PROD cycle += missing.
            } else {
                cycle += 96; // between PickWildMonNature_forSynch and PickWildMonNature_ifNotSynchro.

                let nature_rand_val = rng.rand::<u16>();
                nature_rand = (nature_rand_val % 25) as u8;
                cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25); 
                cycle += 179; // between PickWildMonNature_ifNotSynchro and CreateMonWithNature_pidlow.

                if VERBOSE { println!("Not synchronize: {:?}", nature_rand); }
            }
        }
    };

    let pid_reroll_cycle_start = cycle;
    let mut pid: u32;
    loop {
        gen_mons.pid_reroll_count += 1;
        let pid_low = rng.rand::<u16>() as u32;

        let method3_or_5_range = 80;
        if let (method3, Some(gen_mon)) = generate_3wild_method3_or_5(rng.clone(), settings, pid_low, nature_rand) {
            let gen_mon_cycle = GeneratedPokemonCycle { 
                cycle_start: cycle, 
                cycle_end: cycle + method3_or_5_range, 
                gen_mon
            };
            if method3 {
                gen_mons.method3.push(gen_mon_cycle);
            } else {
                gen_mons.add_method5(gen_mon_cycle);
            }
        }
        cycle += method3_or_5_range;  //between CreateMonWithNature_pidlow to CreateMonWithNature_pidhigh

        let pid_high = rng.rand::<u16>() as u32;
        pid = (pid_high << 16) | pid_low;
        if pid % 25 == nature_rand as u32 {
            // success path cycle increment is done outside of this loop
            break;
        }

        let method5_range = 140 + calc_modulo_cycle_u(pid, 25);
        
        if let Some(gen_mon) = generate_3wild_method5(rng.clone(), settings, nature_rand) {
            gen_mons.method5.push(GeneratedPokemonCycle {
                cycle_start:cycle,
                cycle_end:cycle + method5_range,
                gen_mon
            });
        }
        cycle += method5_range;
    }
    gen_mons.pid_reroll_cycle = cycle - pid_reroll_cycle_start;


    let method2_range = calc_modulo_cycle_u(pid, 25) 
        + 100 * calc_modulo_cycle_u(pid, 24)
        + 36721; // between CreateMonWithNature_pidhigh and CreateBoxMon_ivs1

    if let Some(gen_mon) = generate_3wild_method2(rng.clone(), settings, pid, nature_rand) {
        gen_mons.method2 = Some((GeneratedPokemonCycle {
            cycle_start:cycle,
            cycle_end:cycle + method2_range,
            gen_mon
        }));
    }
    cycle += method2_range;

    let iv1 = rng.rand::<u16>();

    let method4_range = 36 * calc_modulo_cycle_u(pid, 24)
        + 11103; // between CreateBoxMon_ivs1 and CreateBoxMon_ivs2

    if let Some(gen_mon) = generate_3wild_method4(rng.clone(), settings, pid, nature_rand, iv1) {
        gen_mons.method4 = Some((GeneratedPokemonCycle {
            cycle_start:cycle,
            cycle_end:cycle + method4_range,
            gen_mon
        }));
    }
    cycle += method4_range;

    let iv2 = rng.rand::<u16>();
    let ivs = Ivs::new_g3(iv1, iv2);

    if let Some(gen_mon) = generate_3wild_end(settings, pid, ivs, nature_rand) {
        gen_mons.method1 = Some((GeneratedPokemonCycle {
            cycle_start:cycle,
            cycle_end:280_896,
            gen_mon
        }));
    }
    gen_mons
}

pub fn generate_3wild_method2(
    mut rng: Pokerng,
    settings: &Gen3WOpts,
    pid: u32,
    nature_rand: u8,
) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method2

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    generate_3wild_end(settings, pid, ivs, nature_rand)
}

pub fn generate_3wild_method4(
    mut rng: Pokerng,
    settings: &Gen3WOpts,
    pid: u32,
    nature_rand: u8,
    iv1: u16,
) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method4

    let ivs = Ivs::new_g3(iv1, rng.rand::<u16>());

    generate_3wild_end(settings, pid, ivs, nature_rand)
}

pub fn generate_3wild_method3_or_5(
    mut rng: Pokerng,
    settings: &Gen3WOpts,
    pid_low: u32,
    nature_rand: u8,
) -> (bool, Option<GeneratedPokemon>) {
    rng.rand::<u16>(); // Vblank from method3
    let pid_high = rng.rand::<u16>() as u32;
    let mut pid = (pid_high << 16) | pid_low;
    let mut method3 = true;
    if pid % 25 != nature_rand as u32 {
        method3 = false;
        loop {
            let pid_low = rng.rand::<u16>() as u32;
            let pid_high = rng.rand::<u16>() as u32;
            pid = (pid_high << 16) | pid_low;
            if pid % 25 == nature_rand as u32 {
                break;
            }
        }
    }
    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    (method3, generate_3wild_end(settings, pid, ivs, nature_rand))
}

pub fn generate_3wild_method5(
    mut rng: Pokerng,
    settings: &Gen3WOpts,
    nature_rand: u8,
) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method5

    let mut pid: u32;
    loop {
        let pid_low = rng.rand::<u16>() as u32;
        let pid_high = rng.rand::<u16>() as u32;
        pid = (pid_high << 16) | pid_low;
        if pid % 25 == nature_rand as u32 {
            break;
        }
    }

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    generate_3wild_end(settings, pid, ivs, nature_rand)
}

pub fn generate_3wild_end(
    settings: &Gen3WOpts,
    pid: u32,
    ivs: Ivs,
    nature_rand: u8,
) -> Option<GeneratedPokemon> {
    // Filters
    let shiny = gen3_shiny(pid, settings.tid, settings.sid);
    if let Some(wanted) = settings.shiny_type {
        if (shiny && wanted == ShinyType::NotShiny) || (!shiny && wanted != ShinyType::NotShiny) {
            return None;
        }
    }

    let ability = AbilityType::from_gen3_pid(pid);
    if let Some(wanted_ability) = settings.filter.ability {
        if ability != wanted_ability {
            return None;
        }
    }
    let rate: u8 = (pid & 0xFF) as u8;
    let gender = GenderRatio::gender(&settings.gender_ratio, rate);
    if let Some(wanted_gender) = settings.filter.gender {
        if gender != wanted_gender {
            return None;
        }
    }

    if !Ivs::filter(&ivs, &settings.filter.min_ivs, &settings.filter.max_ivs) {
        return None;
    }

    let nature = Nature::from(nature_rand);
    if let Some(wanted_nature) = settings.filter.nature {
        if nature != wanted_nature {
            return None;
        }
    }

    Some(GeneratedPokemon {
        pid,
        shiny,
        ability,
        gender,
        ivs,
        nature,
        advance: 0,
        encounter_slot: EncounterSlot::Slot0,
        synch: false,
    })
}

#[cfg(test)]
mod test {

    use super::*;

    fn new_opts() -> Gen3WOpts {
        Gen3WOpts {
            shiny_type: None,
            tid: 0,
            sid: 0,
            gender_ratio: GenderRatio::OneToOne,
            encounter_slot: None,
            method: Gen3Method::H1,
            initial_advances: 0,
            max_advances: 9,
            synchronize:Some(Gen3Lead::Synchronize((Nature::Hardy))),
            filter: PkmFilter {
                shiny: false,
                nature: None,
                gender: None,
                min_ivs: Ivs {
                    hp: 0,
                    atk: 0,
                    def: 0,
                    spa: 0,
                    spd: 0,
                    spe: 0,
                },
                max_ivs: Ivs {
                    hp: 31,
                    atk: 31,
                    def: 31,
                    spa: 31,
                    spd: 31,
                    spe: 31,
                },
                ability: None,
                stats: None,
            },
        }
    }

    #[test]
    fn gen_mons_one() {
        let seed = 0;
        let options = new_opts();

        let mut rng = Pokerng::new(seed);

        //adv X in ChooseWildMonIndex_Land is triggered if seed is advanced by X + 1
        // if i want rng.advance(5577); i want ChooseWildMonIndex_Land 

        rng.advance(0);
        
        let gen_mons = generate_pokemons(&mut rng, &options);
        println!("{:?}", gen_mons);
        assert!(false);
    }

    #[test]
    fn old_gen_mons() {
        let seed = 0;
        let options = new_opts();

        let mut rng = Pokerng::new(seed);

        //adv X in ChooseWildMonIndex_Land is triggered if seed is advanced by X + 1
        // if i want rng.advance(5577); i want ChooseWildMonIndex_Land 

        let mut max:usize = 0;
        rng.advance(0);
        for i in 0..10000 {
            let gen_mons = generate_pokemons(&mut rng.clone(), &options);
            println!("adv={}, reroll count = {}", i, gen_mons.pid_reroll_count);

            if gen_mons.pid_reroll_cycle > max {
                println!("max cycle: {}, count: {}, adv={}", gen_mons.pid_reroll_cycle, gen_mons.pid_reroll_count, i);
                max = gen_mons.pid_reroll_cycle;
            }
            rng.advance(1);
        }
        assert!(false);

        let gen_mons = generate_pokemons(&mut rng, &options);
        println!("{:?}", gen_mons);
        assert!(false);
    }
}
