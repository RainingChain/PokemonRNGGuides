use crate::{GenderRatio, Ivs, Nature};
use super::{Gen3Lead, Gen3Method, generate_3wild, EncounterSlot, Gen3WOpts};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(
    Default, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, Tsify, Serialize, Deserialize,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[repr(u8)]
pub enum Game {
    #[default]
    Ruby = 0,
    Sapphire = 1,
    Emerald = 2,
}


#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, Tsify, Serialize, Deserialize,)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[repr(u8)]
pub enum Pokemon {
    #[default]
    Lotad = 0,
    Seedot = 1,
    //TODO: Barboach,  Shroomish
}

#[derive(Clone, Debug)]
pub struct Setup {
    pub seed:u32, 
    pub encounter_slot:Vec<EncounterSlot>, 
    pub method:Gen3Method, 
    pub synchronize:Option<(Gen3Lead, Nature)>,
    pub swarm:bool,
} 

pub fn get_size_scale(pid:u32, ivs:&Ivs) -> u16 {
    let hp =  (ivs.hp & 0b1111) as u16;
    let atk = (ivs.atk & 0b1111) as u16;
    let def = (ivs.def & 0b1111) as u16;
    let spa = (ivs.spa & 0b1111) as u16;
    let spd = (ivs.spd & 0b1111) as u16;
    let spe = (ivs.spe & 0b1111) as u16;

    let low_pid = (pid & 0xFF) as u16;
    let high_pid = ((pid & 0xFF00) >> 8) as u16;
    256 * (low_pid ^ (hp * (atk ^ def))) + (high_pid ^ (spe * (spa ^ spd)))
}

pub fn is_largest(pid:u32, ivs:&Ivs) -> bool {
    // scale of 65534 and 65535 are displayed the same in-game
    get_size_scale(pid, ivs) >= 65534
}

pub fn get_earliest_advance_for_max_size_for_setup(setup: &Setup) -> usize {
    for i in 0..10000 {
        let opts = Gen3WOpts {
            shiny_type: None,
            ability: None,
            gender: None,
            nature: None,
            iv_range: (Ivs::create_min(), Ivs::create_max()),
            tid: 0, // doesn't matter
            sid: 0, // doesn't matter
            gender_ratio: GenderRatio::OneToOne,
            encounter_slot:Some(setup.encounter_slot.clone()),
            method: Some(setup.method),
            min_advances: i * 10000,
            max_advances: (i + 1) * 10000,
            synchronize:setup.synchronize,
            swarm:setup.swarm,
        };
        let generated_pokemons = generate_3wild(&opts, setup.seed);
        let largest = generated_pokemons.iter().find(|poke|{
            is_largest(poke.pid, &poke.ivs)
        });

        if let Some(largest) = largest {
            return largest.advances
        }
    }
    // error
    0
}


pub fn get_earliest_advance_for_max_size_for_all_setups(game:Game, pokemon:Pokemon) -> Vec<(Setup, usize)> {
    /*
    Lotad: 
        Route 102: {Ruby: None}, {Sapphire: 4,5 20%}, {Emerald: 4,5 20%}
        Route 114: {Ruby: None}, {Sapphire: 1,4 30%}, {Emerald: 1,4 30%}

    Seedot:
        Route 102: {Ruby: 4,5 20%}, {Sapphire: None}, {Emerald: 11 1%} + Swarm
        Route 114: {Ruby: 1,4 30%}, {Sapphire: None}, {Emerald: None}
        Route 117: {Ruby: None},    {Sapphire: None}, {Emerald: Emerald: 11 1%} + Swarm
        Route 120: {Ruby: None},    {Sapphire: None}, {Emerald: Emerald: 11 1%} + Swarm
    */
    let encounter_slot_swarm_possible_values = match game {
        Game::Ruby => {
            match pokemon {
                Pokemon::Lotad => vec![],
                Pokemon::Seedot => vec![
                    (vec![EncounterSlot::Slot4,EncounterSlot::Slot5], true), // Route 102
                    (vec![EncounterSlot::Slot1,EncounterSlot::Slot4], false), // Route 114
                ],
            }
        },
        Game::Sapphire => {
            match pokemon {
                Pokemon::Lotad => vec![
                    (vec![EncounterSlot::Slot4,EncounterSlot::Slot5], false), // Route 102
                    (vec![EncounterSlot::Slot1,EncounterSlot::Slot4], false), // Route 114
                ],
                Pokemon::Seedot => vec![
                    (vec![], true), // Route 102, 117, 120
                ],
            }
        },
        Game::Emerald => {
            match pokemon {
                Pokemon::Lotad => vec![
                    (vec![EncounterSlot::Slot4,EncounterSlot::Slot5], false), // Route 102
                    (vec![EncounterSlot::Slot1,EncounterSlot::Slot4], false), // Route 114
                ],
                Pokemon::Seedot => vec![
                    (vec![EncounterSlot::Slot11], true) // Route 102, 117, 120
                ],
            }
        }
    };

    if encounter_slot_swarm_possible_values.is_empty() {
        return vec![];
    }

    let seed = match game {
        Game::Ruby | Game::Sapphire => 0x5A0,
        Game::Emerald => 0,
    };

    let synchronize_possible_values:Vec<Option<(Gen3Lead, Nature)>> = match game {
        Game::Ruby | Game::Sapphire => vec![None],
        Game::Emerald => {
            (0u8..26).map(|nature|{
                let nature:Option<Nature> = if nature == 25 { None } else { Some(nature.into()) };
                match nature {
                    None => None,
                    Some(nature) => Some((Gen3Lead::Synchronize, nature))
                }
            }).collect()
        },
    }; 

    let mut setups:Vec<Setup> = vec![];
    for (encounter_slot, swarm) in encounter_slot_swarm_possible_values {
        let swarm_possible_values = if swarm { vec![false, true] } else { vec![false] };

        for synchronize in synchronize_possible_values.iter() {
            for method in vec![Gen3Method::H1,Gen3Method::H2,Gen3Method::H4] {
                for swarm in swarm_possible_values.iter() {
                    setups.push(Setup {
                        seed,
                        encounter_slot:encounter_slot.clone(), 
                        method, 
                        synchronize:*synchronize,
                        swarm:*swarm,
                    });
                }
            }
        }
    }

    let mut res:Vec<(Setup, usize)> = setups.iter().map(|setup|{
        (setup.clone(), get_earliest_advance_for_max_size_for_setup(setup))
    }).collect();

    res.sort_by(|res0, res1|{
        res0.1.cmp(&res1.1)
    });

    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_size_scale() {
        assert_eq!(get_size_scale(0xFF123432, &Ivs { hp:31, atk:1, def:6,spa:1,spd:5,spe:16}), 23348);
    }

    // cargo test --release test_get_size_scale2 -- --nocapture
    #[test]
    fn test_get_size_scale2() {
        //let res = get_earliest_advance_for_max_size_for_all_setups(Game::Emerald, Pokemon::Lotad);
        //9439 => Setup { seed: 0, encounter_slot: [Slot4, Slot5], method: H4, synchronize: Some((Synchronize, Jolly)) }

        //let res = get_earliest_advance_for_max_size_for_all_setups(Game::Emerald, Pokemon::Seedot);
        //291528 => Setup { seed: 0, encounter_slot: [Slot11], method: H2, synchronize: Some((Synchronize, Hardy)) }

        // let res = get_earliest_advance_for_max_size_for_all_setups(Game::Ruby, Pokemon::Seedot);
        // 100149 => Setup { seed: 1440, encounter_slot: [Slot4, Slot5], method: H1, synchronize: None }
        // 122955 => Setup { seed: 1440, encounter_slot: [Slot4, Slot5], method: H2, synchronize: None }

        let res = get_earliest_advance_for_max_size_for_all_setups(Game::Sapphire, Pokemon::Lotad);
        // 100149 => Setup { seed: 1440, encounter_slot: [Slot4, Slot5], method: H1, synchronize: None }
        // 122955 => Setup { seed: 1440, encounter_slot: [Slot4, Slot5], method: H2, synchronize: None }

        for r in res {
            println!("{} => {:?}", r.1, r.0);
        }
        assert!(false);
    }
}
