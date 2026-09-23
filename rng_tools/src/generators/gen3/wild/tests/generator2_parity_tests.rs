use itertools::iproduct as products;

use super::*;
use crate::{
    Species,
    gen3::{Gen3Method, Wild3SpecialEncounterGameData, generate_gen3_wild_old},
};

#[track_caller]
fn assert_mon_results_eq_unordered(
    actual: &[Wild3GeneratorMonResult],
    expected: &[Wild3GeneratorMonResult],
    context: &str,
) {
    assert_eq!(actual.len(), expected.len(), "{context}");
    let mut unmatched: Vec<_> = expected.iter().collect();
    for result in actual {
        let index = unmatched.iter().position(|expected| *expected == result);
        let Some(index) = index else {
            panic!("{context}: unexpected result {result:?}; unmatched results: {unmatched:?}");
        };
        unmatched.swap_remove(index);
    }
}

#[test]
fn method5_cycle_ranges_match_existing_generation() {
    let opts = Wild3GeneratorOptions {
        methods: vec![Gen3Method::Wild5],
        consider_cycles: true,
        consider_rng_manipulated_lead_pid: true,
        ..Default::default()
    };
    let map = Wild3MapGameData::default();
    let rng = Pokerng::with_advances(0, 4894);
    let old = generate_gen3_wild_old(rng, &opts, &map);
    let new = generate_wild3(rng, &opts, &map);
    assert_mon_results_eq_unordered(&new.mon_results, &old.mon_results, "Method 5");
}

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
    for (action, lead, seed, consider_cycles) in products!(
        [Wild3Action::SweetScentLand, Wild3Action::SweetScentWater],
        [
            Gen3Lead::Vanilla,
            Gen3Lead::Static,
            Gen3Lead::MagnetPull,
            Gen3Lead::HustleVitalSpiritPressure,
        ],
        0..32,
        [false, true],
    ) {
        let opts = Wild3GeneratorOptions {
            action,
            lead,
            methods: vec![Gen3Method::Wild1],
            roamer_state: Wild3RoamerState::ActiveInMapLatios,
            mass_outbreak_state: Wild3MassOutbreakState::Route102Seedot,
            consider_cycles,
            generate_even_if_impossible: true,
            ..Default::default()
        };
        let old = generate_gen3_wild_old(Pokerng::new(seed), &opts, &map);
        let new = generate_wild3(Pokerng::new(seed), &opts, &map);
        let context = format!("{action:?} {lead:?} {seed} {consider_cycles}");
        assert_mon_results_eq_unordered(
            &new.mon_results,
            &old.mon_results,
            &context,
        );
        // The old generator rolls a level for roamers; the new one uses their fixed level.
        let is_roamer = new.mon_results.iter().any(|result| {
            matches!(result.encounter_idx, Wild3EncounterIndex::Roamer(_))
        });
        let expected_cycle = if consider_cycles && !is_roamer {
            old.cycle_counter.cycle
        } else {
            CycleAndModCount::default()
        };
        assert_eq!(new.cycle_counter.cycle, expected_cycle, "{context}");
        assert!(new.cycle_counter.cycle_at_moments.is_empty(), "{context}");
    }
}

#[test]
fn fishing_and_rock_smash_match_existing_generation() {
    let mut map = Wild3MapGameData::default();
    map.feebas = Some(Wild3EncounterGameData::default());
    for (action, feebas_state, seed, consider_cycles) in products!(
        [
            Wild3Action::OldRod,
            Wild3Action::GoodRod,
            Wild3Action::SuperRod,
            Wild3Action::RockSmash,
        ],
        [
            Wild3FeebasState::NotInMap,
            Wild3FeebasState::OnFeebasTile,
            Wild3FeebasState::InMapButNotOnFeebasTile,
        ],
        0..64,
        [false, true],
    ) {
        let opts = Wild3GeneratorOptions {
            action,
            feebas_state,
            methods: vec![Gen3Method::Wild1],
            consider_cycles,
            generate_even_if_impossible: true,
            ..Default::default()
        };
        let old = generate_gen3_wild_old(Pokerng::new(seed), &opts, &map);
        let new = generate_wild3(Pokerng::new(seed), &opts, &map);
        let context = format!("{action:?} {feebas_state:?} {seed} {consider_cycles}");
        assert_mon_results_eq_unordered(
            &new.mon_results,
            &old.mon_results,
            &context,
        );
        let expected_cycle = if consider_cycles {
            old.cycle_counter.cycle
        } else {
            CycleAndModCount::default()
        };
        assert_eq!(new.cycle_counter.cycle, expected_cycle, "{context}");
    }
}

#[test]
fn all_methods_and_cycle_ranges_match_existing_generation() {
    let mut map = Wild3MapGameData::default();
    map.is_safari = true;
    for (action, lead, seed) in products!(
        [
            Wild3Action::SweetScentLand,
            Wild3Action::SweetScentWater,
            Wild3Action::OldRod,
            Wild3Action::GoodRod,
            Wild3Action::SuperRod,
            Wild3Action::RockSmash,
        ],
        [
            Gen3Lead::Egg,
            Gen3Lead::CuteCharm(Gender::Female),
            Gen3Lead::Synchronize(Nature::Jolly),
        ],
        0..8,
    ) {
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
        assert_mon_results_eq_unordered(
            &new.mon_results,
            &old.mon_results,
            &format!("{action:?} {lead:?} {seed}"),
        );
        assert_eq!(new.cycle_counter.cycle, old.cycle_counter.cycle);
    }
}
