use crate::Ivs;
use crate::gen3::EncounterSlot;
use crate::gen3::Gen3Lead;
use crate::gen3::Gen3Method;
use crate::rng::Rng;
use crate::rng::StateIterator;
use crate::rng::lcrng::Pokerng;
use crate::{AbilityType, Gender, GenderRatio, Nature, PkmFilter, ShinyType, gen3_shiny};
use super::{Gen3WOpts, GeneratedPokemon};


#[derive(Debug)]
pub struct GeneratedPokemons {
    pub method1:Vec<(usize,usize, GeneratedPokemon)>,
    pub method2:Vec<(usize,usize, GeneratedPokemon)>,
    pub method3:Vec<(usize,usize, GeneratedPokemon)>,
    pub method4:Vec<(usize,usize, GeneratedPokemon)>,
    pub method5:Vec<(usize,usize, GeneratedPokemon)>,
}

impl GeneratedPokemons {
    #[allow(dead_code)]
    fn new() -> Self {
        GeneratedPokemons {
            method1: Vec::new(),
            method2: Vec::new(),
            method3: Vec::new(),
            method4: Vec::new(),
            method5: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub fn generate_pokemons(rng: &mut Pokerng, settings: &Gen3WOpts) -> GeneratedPokemons {
    let mut cycle:usize = 0;
    
    let mut gen_mons = GeneratedPokemons::new();

    let encounter_rand = ((rng.rand::<u32>() >> 16) % 100) as u8;
    let encounter_slot = EncounterSlot::from_rand(encounter_rand);

    if !EncounterSlot::passes_filter(settings.encounter_slot.as_deref(), encounter_slot) {
        return gen_mons;
    }
    rng.rand::<u32>(); // level


    let nature_rand: u8;

    match settings.synchronize {
        None => {
            nature_rand = (rng.rand::<u16>() % 25) as u8;
        }
        Some(Gen3Lead::Synchronize(lead_nature)) => {
            if (rng.rand::<u16>() & 1) == 0 {
                nature_rand = lead_nature.into();
            } else {
                nature_rand = (rng.rand::<u16>() % 25) as u8;
            }
        }
    };

    cycle += 100;
    
    let mut pid: u32;
    loop {
        let pid_low = rng.rand::<u16>() as u32;
        
        if let Some(res) = generate_3wild_method3(rng.clone(), settings, pid_low, nature_rand) {
            gen_mons.method3.push((cycle, cycle + 200, res));
        }
        cycle += 200;
        
        let pid_high = rng.rand::<u16>() as u32;
        pid = (pid_high << 16) | pid_low;
        if pid % 25 == nature_rand as u32 {
            break;
        }

        if let Some(res) = generate_3wild_method5(rng.clone(), settings, nature_rand) {
            gen_mons.method5.push((cycle, cycle + 200, res));
        }
    }


    if let Some(res) = generate_3wild_method2(rng.clone(), settings, pid, nature_rand) {
        gen_mons.method2.push((cycle, cycle + 200, res));
    }

    let iv1= rng.rand::<u16>();
    
    if let Some(res) = generate_3wild_method4(rng.clone(), settings, pid, nature_rand, iv1) {
        gen_mons.method4.push((cycle, cycle + 200, res));
    }

    let iv2 = rng.rand::<u16>();
    let ivs = Ivs::new_g3(iv1, iv2);

    if let Some(res) = generate_3wild_end(settings, pid, ivs, nature_rand) {
        gen_mons.method1.push((cycle, cycle + 200, res));
    }
    gen_mons
}

pub fn generate_3wild_method2(mut rng:Pokerng, settings: &Gen3WOpts, pid: u32, nature_rand:u8) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method2

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    generate_3wild_end(settings, pid, ivs, nature_rand)
}

pub fn generate_3wild_method4(mut rng:Pokerng, settings: &Gen3WOpts, pid: u32, nature_rand:u8, iv1:u16) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method4

    let ivs = Ivs::new_g3(iv1, rng.rand::<u16>());

    generate_3wild_end(settings, pid, ivs, nature_rand)
}

pub fn generate_3wild_method3(mut rng:Pokerng, settings: &Gen3WOpts, pid_low: u32, nature_rand:u8) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method3
    let pid_high = rng.rand::<u16>() as u32;
    let mut pid = (pid_high << 16) | pid_low;
    if pid % 25 != nature_rand as u32 {
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

    generate_3wild_end(settings, pid, ivs, nature_rand)

}


pub fn generate_3wild_method5(mut rng:Pokerng, settings: &Gen3WOpts, nature_rand:u8) -> Option<GeneratedPokemon> {
    rng.rand::<u16>(); // Vblank from method5

    let mut pid:u32;
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


pub fn generate_3wild_end(settings: &Gen3WOpts, pid:u32, ivs:Ivs, nature_rand:u8) -> Option<GeneratedPokemon> {
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
        encounter_slot:EncounterSlot::Slot0,
        synch: false,
    })

}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn gen_mons() {
        let seed = 0;
        let options = Gen3WOpts {
            shiny_type: None,
            tid: 0,
            sid: 0,
            gender_ratio: GenderRatio::OneToOne,
            encounter_slot: None,
            method: Gen3Method::H1,
            initial_advances: 0,
            max_advances: 9,
            synchronize: None,
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
        };

        let gen_mons = generate_pokemons(&mut Pokerng::new(seed), &options);
        println!("{:?}", gen_mons);
        assert!(false);
    }

}
