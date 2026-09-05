use crate::{
    EncounterSlot, Ivs, Nature, PkmFilter,
    gen3::{
        CycleRange, Gen3Lead, Gen3Method, INFINITE_CYCLE, MinMax, VBLANK_BY_FEEBAS_CYCLE_COUNT,
        VBLANK_FREQ, Wild3Action, Wild3EncounterIndex, Wild3GeneratorMonResult,
        Wild3GeneratorOptions, Wild3MapGameData, generate_gen3_wild,
        get_min_mid_max_pre_sweet_scent_cycle, get_min_mid_max_vblank_cycle_duration,
        wild::generator::apply_cycles_causing_vblanks_on_cycle_counter,
    },
    rng::lcrng::Pokerng,
};

#[test]
fn test_generate_wild3_cycle_method_3() {
    let options = Wild3GeneratorOptions {
        methods: vec![Gen3Method::Wild3],
        lead: Gen3Lead::Synchronize(Nature::Serious),
        consider_cycles: true,
        consider_rng_manipulated_lead_pid: true,
        ..Default::default()
    };

    let result = generate_gen3_wild(
        Pokerng::with_advances(0, 3012),
        &options,
        &Wild3MapGameData::default(),
    )
    .mon_results;
    let expected_result = vec![
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 1459093362,
            ivs: Ivs::new(13, 14, 2, 20, 14, 15),
            method: Gen3Method::Wild3,
            cycle_range: Some(CycleRange::new(144256, 81, 80)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 3087365287,
            ivs: Ivs::new(21, 3, 3, 11, 15, 19),
            method: Gen3Method::Wild3,
            cycle_range: Some(CycleRange::new(158414, 81, 80)),
            ..Default::default()
        },
    ];
    assert_eq!(result, expected_result);
}

#[test]
fn test_generate_wild3_cycle_method_3_no_rng_lead_pid() {
    // Same as test_generate_wild3_cycle_method_3, but consider_rng_manipulated_lead_pid is false.
    // This should return an empty result, as the method cannot be triggered with a common lead PID.
    let options = Wild3GeneratorOptions {
        methods: vec![Gen3Method::Wild3],
        lead: Gen3Lead::Synchronize(Nature::Serious),
        filter: PkmFilter::new_allow_all(),
        consider_cycles: true,
        consider_rng_manipulated_lead_pid: false,
        ..Default::default()
    };

    let result = generate_gen3_wild(
        Pokerng::with_advances(0, 3013),
        &options,
        &Wild3MapGameData::default(),
    )
    .mon_results;
    assert_eq!(result, vec![]);
}

#[test]
fn test_generate_wild3_cycle_method_5() {
    let options = Wild3GeneratorOptions {
        methods: vec![Gen3Method::Wild5],
        consider_cycles: true,
        consider_rng_manipulated_lead_pid: true,
        ..Default::default()
    };

    let result = generate_gen3_wild(
        Pokerng::with_advances(0, 4894),
        &options,
        &Wild3MapGameData::default(),
    )
    .mon_results;
    let expected_result = vec![
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 1946911046,
            ivs: Ivs::new(4, 29, 8, 25, 7, 14),
            method: Gen3Method::Wild5,
            cycle_range: Some(CycleRange::new(119019, 80, 20212)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 26625321,
            ivs: Ivs::new(21, 2, 31, 1, 18, 19),
            method: Gen3Method::Wild5,
            cycle_range: Some(CycleRange::new(139231, 80, 7094)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 2210948146,
            ivs: Ivs::new(13, 0, 19, 1, 12, 6),
            method: Gen3Method::Wild5,
            cycle_range: Some(CycleRange::new(146325, 80, 3042)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot5),
            pid: 2335347696,
            ivs: Ivs::new(13, 12, 19, 0, 1, 29),
            method: Gen3Method::Wild5,
            cycle_range: Some(CycleRange::new(149367, 80, 6009)),
            ..Default::default()
        },
    ];
    assert_eq!(result, expected_result);
}

#[test]
fn test_generate_wild3_cycle_methods_1_2_4() {
    let options = Wild3GeneratorOptions {
        methods: vec![Gen3Method::Wild1, Gen3Method::Wild2, Gen3Method::Wild4],
        lead: Gen3Lead::Synchronize(Nature::Hardy),
        consider_cycles: true,
        consider_rng_manipulated_lead_pid: true,
        ..Default::default()
    };

    let result = generate_gen3_wild(
        Pokerng::with_advances(0, 3001),
        &options,
        &Wild3MapGameData::default(),
    )
    .mon_results;
    let expected_result = vec![
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot3),
            pid: 3864471792,
            ivs: Ivs::new(0, 9, 4, 5, 4, 3),
            method: Gen3Method::Wild2,
            cycle_range: Some(CycleRange::new(54709, 80, 112996)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot3),
            pid: 3864471792,
            ivs: Ivs::new(26, 8, 17, 5, 4, 3),
            method: Gen3Method::Wild4,
            cycle_range: Some(CycleRange::new(167705, 80, 38211)),
            ..Default::default()
        },
        Wild3GeneratorMonResult {
            encounter_idx: Wild3EncounterIndex::Slot(EncounterSlot::Slot3),
            pid: 3864471792,
            ivs: Ivs::new(26, 8, 17, 9, 4, 0),
            method: Gen3Method::Wild1,
            cycle_range: Some(CycleRange::new(205916, 80, INFINITE_CYCLE)),
            ..Default::default()
        },
    ];
    assert_eq!(result, expected_result);
}

#[test]
fn test_generate_wild3_feebas_vblank_group() {
    let mut groups: [MinMax; 4] = [
        MinMax::empty(),
        MinMax::empty(),
        MinMax::empty(),
        MinMax::empty(),
    ];

    for feebas_cycle_count in 0..=800_000 {
        let (min_presweet, _, max_presweet) =
            get_min_mid_max_pre_sweet_scent_cycle(Wild3Action::OldRod);
        let (min_vblank_dur, _, max_vblank_dur) = get_min_mid_max_vblank_cycle_duration();

        let (_, min_vblank_count) = apply_cycles_causing_vblanks_on_cycle_counter(
            min_presweet,
            feebas_cycle_count,
            min_vblank_dur,
        );
        let (_, max_vblank_count) = apply_cycles_causing_vblanks_on_cycle_counter(
            max_presweet,
            feebas_cycle_count,
            max_vblank_dur,
        );

        groups[min_vblank_count].update_bounds(feebas_cycle_count);
        groups[max_vblank_count].update_bounds(feebas_cycle_count);
    }

    assert_eq!(groups, VBLANK_BY_FEEBAS_CYCLE_COUNT);
}
