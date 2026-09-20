use super::*;
use crate::{
    Species,
    gen3::{Gen3Method, Wild3SpecialEncounterGameData, generate_gen3_wild_old},
};

#[test]
fn sweet_scent_matches_existing_generation() {
    let mut map = Wild3MapGameData::default();
    map.slots_by_action[Wild3Action::SweetScentLand as usize][0]
        .species_data
        .species = Species::Magnemite;
    map.slots_by_action[Wild3Action::SweetScentLand as usize][1]
        .species_data
        .species = Species::Aron;
    map.roamers.push(Wild3SpecialEncounterGameData {
        id: Wild3RoamerState::ActiveInMapLatios,
        ..Default::default()
    });
    map.mass_outbreaks.push(Wild3SpecialEncounterGameData {
        id: Wild3MassOutbreakState::Route102Seedot,
        ..Default::default()
    });
    for action in [Wild3Action::SweetScentLand, Wild3Action::SweetScentWater] {
        for lead in [
            Gen3Lead::Vanilla,
            Gen3Lead::Static,
            Gen3Lead::MagnetPull,
            Gen3Lead::HustleVitalSpiritPressure,
        ] {
            for seed in 0..32 {
                let opts = Wild3GeneratorOptions {
                    action,
                    lead,
                    methods: vec![Gen3Method::Wild1],
                    roamer_state: Wild3RoamerState::ActiveInMapLatios,
                    mass_outbreak_state: Wild3MassOutbreakState::Route102Seedot,
                    ..Default::default()
                };
                let old = generate_gen3_wild_old(Pokerng::new(seed), &opts, &map);
                let new = generate_wild3(Pokerng::new(seed), &opts, &map);
                assert_eq!(
                    new.mon_results, old.mon_results,
                    "{action:?} {lead:?} {seed}"
                );
                assert_eq!(new.cycle_counter.cycle, old.cycle_counter.cycle);
                assert_eq!(
                    new.cycle_counter.cycle_at_moments[0].moment,
                    Moment::TrySweetScentEncounter
                );
            }
        }
    }
}

#[test]
fn fishing_and_rock_smash_match_existing_generation() {
    let mut map = Wild3MapGameData::default();
    map.feebas = Some(Wild3EncounterGameData::default());
    for action in [
        Wild3Action::OldRod,
        Wild3Action::GoodRod,
        Wild3Action::SuperRod,
        Wild3Action::RockSmash,
    ] {
        for feebas_state in [
            Wild3FeebasState::NotInMap,
            Wild3FeebasState::OnFeebasTile,
            Wild3FeebasState::InMapButNotOnFeebasTile,
        ] {
            for seed in 0..64 {
                let opts = Wild3GeneratorOptions {
                    action,
                    feebas_state,
                    methods: vec![Gen3Method::Wild1],
                    ..Default::default()
                };
                let old = generate_gen3_wild_old(Pokerng::new(seed), &opts, &map);
                let new = generate_wild3(Pokerng::new(seed), &opts, &map);
                assert_eq!(
                    new.mon_results, old.mon_results,
                    "{action:?} {feebas_state:?} {seed}"
                );
                assert_eq!(new.cycle_counter.cycle, old.cycle_counter.cycle);
            }
        }
    }
}

#[test]
fn all_methods_and_cycle_ranges_match_existing_generation() {
    let mut map = Wild3MapGameData::default();
    map.is_safari = true;
    for action in [
        Wild3Action::SweetScentLand,
        Wild3Action::SweetScentWater,
        Wild3Action::OldRod,
        Wild3Action::GoodRod,
        Wild3Action::SuperRod,
        Wild3Action::RockSmash,
    ] {
        for lead in [
            Gen3Lead::Egg,
            Gen3Lead::CuteCharm(Gender::Female),
            Gen3Lead::Synchronize(Nature::Jolly),
        ] {
            for seed in 0..8 {
                let opts = Wild3GeneratorOptions {
                    action,
                    lead,
                    methods: vec![
                        Gen3Method::Wild1,
                        Gen3Method::Wild2,
                        Gen3Method::Wild3,
                        Gen3Method::Wild4,
                        Gen3Method::Wild5,
                    ],
                    consider_cycles: true,
                    generate_even_if_impossible: true,
                    ..Default::default()
                };
                let old = generate_gen3_wild_old(Pokerng::new(seed), &opts, &map);
                let new = generate_wild3(Pokerng::new(seed), &opts, &map);
                assert_eq!(
                    new.mon_results, old.mon_results,
                    "{action:?} {lead:?} {seed}"
                );
                assert_eq!(new.cycle_counter.cycle, old.cycle_counter.cycle);
            }
        }
    }
}
