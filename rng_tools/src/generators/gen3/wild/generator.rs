use super::{Gen3EncounterType, calc_modulo_cycle_s, calc_modulo_cycle_u};
use crate::EncounterSlot;
use crate::Ivs;
use crate::Species;
use crate::gen3::{Gen3Lead, Gen3Method};
use crate::rng::Rng;
use crate::rng::lcrng::Pokerng;
use crate::{AbilityType, Gender, GenderRatio, Nature, PkmFilter, gen3_shiny};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Wild3EncounterSlotInfo {
    pub min_level: u8,
    pub max_level: u8,
    pub species: Species,
    pub gender_ratio: GenderRatio,
    pub is_electric_type: bool,
    pub is_steel_type: bool,
}

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Wild3EncounterTable {
    pub map_id: String,
    pub encounter_type: Gen3EncounterType,
    pub slots: Vec<Wild3EncounterSlotInfo>,
}

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Wild3GeneratorOptions {
    pub advance: usize,
    pub tid: u16,
    pub sid: u16,
    pub gender_ratio: GenderRatio,
    pub map_idx: usize,
    pub encounter_slot: Option<Vec<EncounterSlot>>,
    pub methods: Vec<Gen3Method>,
    pub lead: Option<Gen3Lead>,
    pub filter: PkmFilter,
}

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Wild3GeneratorResult {
    pub encounter_slot: EncounterSlot,
    pub pid: u32,
    pub ivs: Ivs,
    pub method: Gen3Method,
    pub cycle_range: (usize, usize),
}

const BASE_LEAD_PID: u32 = 0;

/*
SweetScentWildEncounter
Random in ChooseWildMonIndex_Land
Random in ChooseWildMonLevel
Random in ChooseWildMonLevel_levelRange
Random in PickWildMonNature_pickRandom
Random in CreateMonWithNature_pidlow
Random in CreateMonWithNature_pidhigh
CreateMon
Random in CreateBoxMon_ivs1
Random in CreateBoxMon_ivs2
*/


pub fn generate_gen3_wild(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
) -> Vec<Wild3GeneratorResult> {
    let mut results: Vec<Wild3GeneratorResult> = vec![];

    let mut cycle: usize = 0;

    // between SweetScentWildEncounter and ChooseWildMonIndex_Land
    cycle += 12059 + 32 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
    
    let encounter_rand_val = rng.rand::<u16>() as u32; // ChooseWildMonIndex_Land
    let encounter_rand = (encounter_rand_val % 100) as u8;
    let encounter_slot = EncounterSlot::from_rand(encounter_rand);

    //between ChooseWildMonIndex_Land and ChooseWildMonLevel
    cycle += 378 + calc_modulo_cycle_u(encounter_rand_val, 100); // TODO: cycle increment depends on slot

    if !EncounterSlot::passes_filter(opts.encounter_slot.as_deref(), encounter_slot) {
        return results;
    }

    let lvl_range_rand_val = rng.rand::<u16>(); // ChooseWildMonLevel
    let lvl_range = 1; // TODO: get from Wild3EncounterTable
    
    let mut required_gender:Option<Gender> = None;
    let mut required_nature:Nature = Nature::Hardy; //NO_PROD

    //between ChooseWildMonLevel and CreateMon
    match opts.lead {
        None => {
            required_gender = None;

            // between ChooseWildMonLevel and PickWildMonNature_pickRandom
            cycle += calc_modulo_cycle_s(lvl_range_rand_val as i32, lvl_range);
            cycle += 36 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
            if opts.gender_ratio.has_multiple_genders() {
                cycle += 12 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
            }
            cycle += 30945; 

            // between PickWildMonNature_pickRandom and CreateMonWithNature_pidlow
            let nature_rand_val = rng.rand::<u16>();
            cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25);
            cycle += 179; // between PickWildMonNature_ifNotSynchro and CreateMonWithNature_pidlow.
            required_nature = ((nature_rand_val % 25) as u8).into();
        },
        Some(Gen3Lead::Synchronize(lead_nature)) => {
            required_gender = None;

            // between ChooseWildMonLevel and PickWildMonNature_pickRandom
            cycle += calc_modulo_cycle_s(lvl_range_rand_val as i32, lvl_range);
            cycle += 36 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
            if opts.gender_ratio.has_multiple_genders() {
                cycle += 12 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
            }
            cycle += 30945; 

            if (rng.rand::<u16>() & 1) == 0 {
                required_nature = lead_nature;
                cycle += 389 + calc_modulo_cycle_u(BASE_LEAD_PID, 25);
            } else {
                cycle += 96; // between PickWildMonNature_forSynch and PickWildMonNature_ifNotSynchro.

                // between PickWildMonNature_pickRandom and CreateMonWithNature_pidlow
                let nature_rand_val = rng.rand::<u16>();
                cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25);
                cycle += 179; // between PickWildMonNature_ifNotSynchro and CreateMonWithNature_pidlow.
                required_nature = ((nature_rand_val % 25) as u8).into();

            }

        },
        Some(Gen3Lead::CuteCharm(lead_gender)) => {

            // between ChooseWildMonLevel and CreateWildMon_CuteCharmRandom
            cycle += calc_modulo_cycle_s(lvl_range_rand_val as i32, lvl_range);
            cycle += 20 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
            if opts.gender_ratio.has_multiple_genders() {
                cycle += 12 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
                cycle += 25182;  //NO_PROD
            }

            let cute_charm_rand_val = rng.rand::<u16>();

            // between CreateWildMon_CuteCharmRandom and PickWildMonNature_pickRandom
            cycle += calc_modulo_cycle_u(cute_charm_rand_val as u32, 3);

            if cute_charm_rand_val % 3 != 0 {
                required_gender = Some(if lead_gender == Gender::Female {
                    Gender::Male
                } else {
                    Gender::Female
                });
                cycle += 24 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
                cycle += 8786;

                // between PickWildMonNature_pickRandom and CreateMonWithGenderNatureLetter_pidlow
                let nature_rand_val = rng.rand::<u16>();
                cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25);
                cycle += 223;
                required_nature = ((nature_rand_val % 25) as u8).into();
            } else {
                required_gender = None;
                cycle += 16 * calc_modulo_cycle_u(BASE_LEAD_PID, 24);
                cycle += 5863;

                // between PickWildMonNature_pickRandom and CreateMonWithGenderNatureLetter_pidlow
                let nature_rand_val = rng.rand::<u16>();
                cycle += calc_modulo_cycle_u(nature_rand_val as u32, 25);
                cycle += 179;
                required_nature = ((nature_rand_val % 25) as u8).into();
            }
        },
    }
    
    let methods_contains_wild3 = opts.methods.contains(&Gen3Method::Wild3);
    let methods_contains_wild5 = opts.methods.contains(&Gen3Method::Wild5);

    let mut pid: u32;
    loop {
        let pid_low = rng.rand::<u16>() as u32;

        let method3_range = 80;
        if methods_contains_wild3 {
            if let Some(gen_mon_wild3) = generate_gen3_wild_method3(
                *rng,
                opts,
                encounter_slot,
                pid_low,
                required_gender,
                required_nature,
                (cycle, cycle + 1),
            ) {
                results.push(gen_mon_wild3);
            }
        }
        cycle += method3_range;
        
        let pid_high = rng.rand::<u16>() as u32;
        pid = (pid_high << 16) | pid_low;

        let good_nature = Nature::from_pid(pid) == required_nature;

        let good_gender = if let Some(required_gender) = required_gender {
            let generated_mon_gender = opts.gender_ratio.gender_from_pid(pid);
            generated_mon_gender == required_gender
        } else {
            true
        };

        if good_nature && good_gender {
            // cycle increment done after the loop
            break;
        }

        // between CreateMonWithNature_pidhigh and CreateMonWithNature_pidlow (retry)
        let retry_cycle = if good_nature { 140 } else { 158 }; // 18 cycles to check gender
        let method5_range = retry_cycle + calc_modulo_cycle_u(pid, 25);
        if methods_contains_wild5 {
            // Multiple iterations will result in the same Method5 Pokémon.
            // To avoid duplicates, we add the generated Pokémon only in the latest possible PID reroll.
            if let Some(gen_mon_wild5) = generate_gen3_wild_method5(
                *rng,
                opts,
                encounter_slot,
                required_gender,
                required_nature,
                (0, 0), //NO_PROD
            ) {
                results.push(gen_mon_wild5);
            }
        }
        cycle += method5_range;
    }

    if !passes_pid_filter(opts, pid) {
        return results;
    }

    // between CreateMonWithNature_pidhigh and CreateBoxMon_ivs1
    let method2_range = calc_modulo_cycle_u(pid, 25) 
        + 100 * calc_modulo_cycle_u(pid, 24)
        + 36900;  // TODO: investigate if species impact this. Got 36721, 36903 (poochyena), 36950 (wurmple), 37210 (ralts).

    if opts.methods.contains(&Gen3Method::Wild2) {
        if let Some(gen_mon_wild2) =
            generate_gen3_wild_method2(*rng, opts, encounter_slot, pid, (0, 0))
        {
            results.push(gen_mon_wild2);
        }
    }
    cycle += method2_range;


    let iv1 = rng.rand::<u16>();

    // between CreateBoxMon_ivs1 and CreateBoxMon_ivs2
    let method4_range = 36 * calc_modulo_cycle_u(pid, 24)
        + 11103; // between CreateBoxMon_ivs1 and CreateBoxMon_ivs2

    if opts.methods.contains(&Gen3Method::Wild4) {
        if let Some(gen_mon_wild4) =
            generate_gen3_wild_method4(*rng, opts, encounter_slot, pid, iv1, (0, 0))
        {
            results.push(gen_mon_wild4);
        }
    }
    cycle += method4_range;

    if opts.methods.contains(&Gen3Method::Wild1) {
        let ivs = Ivs::new_g3(iv1, rng.rand::<u16>());

        if let Some(gen_mon_wild1) =
            create_if_passes_filter(opts, pid, ivs, Gen3Method::Wild1, encounter_slot, (0, 0))
        {
            results.push(gen_mon_wild1);
        }
    }

    results
}

pub fn generate_gen3_wild_method2(
    mut rng: Pokerng,
    opts: &Wild3GeneratorOptions,
    encounter_slot: EncounterSlot,
    pid: u32,
    cycle_range: (usize, usize),
) -> Option<Wild3GeneratorResult> {
    rng.rand::<u16>(); // Vblank from method2

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    create_if_passes_filter(
        opts,
        pid,
        ivs,
        Gen3Method::Wild2,
        encounter_slot,
        cycle_range,
    )
}

pub fn generate_gen3_wild_method3(
    mut rng: Pokerng,
    opts: &Wild3GeneratorOptions,
    encounter_slot: EncounterSlot,
    pid_low: u32,
    required_gender: Option<Gender>,
    required_nature: Nature,
    cycle_range: (usize, usize),
) -> Option<Wild3GeneratorResult> {
    rng.rand::<u16>(); // Vblank from method3

    let pid_high = rng.rand::<u16>() as u32;
    let pid = (pid_high << 16) | pid_low;
    if Nature::from_pid(pid) != required_nature {
        return None;
    }
    if let Some(required_gender) = required_gender {
        let generated_mon_gender = opts.gender_ratio.gender_from_pid(pid);
        if generated_mon_gender != required_gender {
            return None;
        }
    }

    if !passes_pid_filter(opts, pid) {
        return None;
    }

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    create_if_passes_filter(
        opts,
        pid,
        ivs,
        Gen3Method::Wild3,
        encounter_slot,
        cycle_range,
    )
}

pub fn generate_gen3_wild_method4(
    mut rng: Pokerng,
    opts: &Wild3GeneratorOptions,
    encounter_slot: EncounterSlot,
    pid: u32,
    iv1: u16,
    cycle_range: (usize, usize),
) -> Option<Wild3GeneratorResult> {
    rng.rand::<u16>(); // Vblank from method4

    let ivs = Ivs::new_g3(iv1, rng.rand::<u16>());

    create_if_passes_filter(
        opts,
        pid,
        ivs,
        Gen3Method::Wild4,
        encounter_slot,
        cycle_range,
    )
}

pub fn generate_gen3_wild_method5(
    mut rng: Pokerng,
    opts: &Wild3GeneratorOptions,
    encounter_slot: EncounterSlot,
    required_gender: Option<Gender>,
    required_nature: Nature,
    cycle_range: (usize, usize),
) -> Option<Wild3GeneratorResult> {
    rng.rand::<u16>(); // Vblank from method5

    let pid_low = rng.rand::<u16>() as u32;
    let pid_high = rng.rand::<u16>() as u32;
    let pid = (pid_high << 16) | pid_low;

    if Nature::from_pid(pid) != required_nature {
        return None;
    }
    if let Some(required_gender) = required_gender {
        let generated_mon_gender = opts.gender_ratio.gender_from_pid(pid);
        if generated_mon_gender != required_gender {
            return None;
        }
    }

    if !passes_pid_filter(opts, pid) {
        return None;
    }

    let ivs = Ivs::new_g3(rng.rand::<u16>(), rng.rand::<u16>());

    create_if_passes_filter(
        opts,
        pid,
        ivs,
        Gen3Method::Wild5,
        encounter_slot,
        cycle_range,
    )
}

pub fn passes_pid_filter(opts: &Wild3GeneratorOptions, pid: u32) -> bool {
    if opts.filter.shiny {
        let generated_shiny = gen3_shiny(pid, opts.tid, opts.sid);
        if !generated_shiny {
            return false;
        }
    }

    if let Some(wanted_ability) = opts.filter.ability {
        let generated_ability = AbilityType::from_gen3_pid(pid);
        if generated_ability != wanted_ability {
            return false;
        }
    }

    if let Some(wanted_gender) = opts.filter.gender {
        let generated_gender = opts.gender_ratio.gender_from_pid(pid);
        if generated_gender != wanted_gender {
            return false;
        }
    }

    if let Some(wanted_nature) = opts.filter.nature {
        let nature = Nature::from_pid(pid);
        if nature != wanted_nature {
            return false;
        }
    }

    true
}

pub fn passes_ivs_filter(opts: &Wild3GeneratorOptions, ivs: &Ivs) -> bool {
    Ivs::filter(ivs, &opts.filter.min_ivs, &opts.filter.max_ivs)
}

pub fn create_if_passes_filter(
    opts: &Wild3GeneratorOptions,
    pid: u32,
    ivs: Ivs,
    method: Gen3Method,
    encounter_slot: EncounterSlot,
    cycle_range: (usize, usize),
) -> Option<Wild3GeneratorResult> {
    if !passes_ivs_filter(opts, &ivs) {
        return None;
    }

    Some(Wild3GeneratorResult {
        pid,
        ivs,
        method,
        encounter_slot,
        cycle_range,
    })
}
