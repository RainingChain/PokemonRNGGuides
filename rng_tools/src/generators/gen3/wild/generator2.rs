#![allow(non_snake_case)]

use super::generator::{
    Wild3GeneratorOptions, Wild3GeneratorResults, generate_wild3_from_encounter,
    handle_feebas_cycle_counter,
};
use super::{calc_modulo_cycle_signed, calc_modulo_cycle_unsigned};
use crate::{
    EncounterSlot,
    gen3::{
        CycleCounter, Gen3Lead, Moment, Wild3Action, Wild3EncounterGameData, Wild3EncounterIndex,
        Wild3FeebasState, Wild3MapGameData, Wild3MassOutbreakState, Wild3RoamerState,
    },
    rng::{Rng, lcrng::Pokerng},
};

// The field-effect task's visual frames are outside the encounter RNG path.
pub fn generate_wild3(
    mut rng: Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
) -> Wild3GeneratorResults {
    let mut cycle_counter = CycleCounter::default();
    match opts.action {
        Wild3Action::SweetScentLand | Wild3Action::SweetScentWater => {
            TrySweetScentEncounter(&mut rng, opts, map_data, &mut cycle_counter)
        }
        Wild3Action::OldRod | Wild3Action::GoodRod | Wild3Action::SuperRod => {
            Fishing_StartEncounter(&mut rng, opts, map_data, &mut cycle_counter)
        }
        Wild3Action::RockSmash => {
            RockSmashWildEncounter(&mut rng, opts, map_data, &mut cycle_counter)
        }
    }
}

fn Fishing_StartEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Wild3GeneratorResults {
    cycle_counter.on_moment_reached(Moment::Fishing_StartEncounter);
    FishingWildEncounter(rng, opts, map_data, cycle_counter)
}

fn FishingWildEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Wild3GeneratorResults {
    cycle_counter.on_moment_reached(Moment::FishingWildEncounter);
    if CheckFeebas(rng, opts, cycle_counter) {
        let Some(encounter) = map_data.feebas.as_ref() else {
            return Wild3GeneratorResults::empty();
        };
        let level = ChooseWildMonLevel(rng, encounter, opts.lead, cycle_counter);
        CreateWildMon(encounter, level, cycle_counter);
        return finish(
            rng,
            opts,
            map_data,
            cycle_counter,
            Wild3EncounterIndex::Feebas,
            Some(level),
        );
    }
    let Some((encounter_idx, level)) = GenerateFishingWildMon(rng, opts, map_data, cycle_counter)
    else {
        return Wild3GeneratorResults::empty();
    };
    finish(
        rng,
        opts,
        map_data,
        cycle_counter,
        encounter_idx,
        Some(level),
    )
}

fn CheckFeebas(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    cycle_counter: &mut CycleCounter,
) -> bool {
    cycle_counter.on_moment_reached(Moment::CheckFeebas);
    if opts.feebas_state == Wild3FeebasState::NotInMap {
        return false;
    }
    if rng.rand::<u16>() % 100 > 49 {
        return false;
    }
    handle_feebas_cycle_counter(rng, cycle_counter, opts.feebas_cycles);
    opts.feebas_state == Wild3FeebasState::OnFeebasTile
}

fn GenerateFishingWildMon(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Option<(Wild3EncounterIndex, u8)> {
    cycle_counter.on_moment_reached(Moment::GenerateFishingWildMon);
    let index = ChooseWildMonIndex_Fishing(rng, opts.action, opts.lead, cycle_counter);
    let encounter = map_data
        .slots_by_action
        .get(opts.action as usize)?
        .get(index)?;
    let level = ChooseWildMonLevel(rng, encounter, opts.lead, cycle_counter);
    CreateWildMon(encounter, level, cycle_counter);
    Some((Wild3EncounterIndex::Slot((index as u8).into()), level))
}

fn ChooseWildMonIndex_Fishing(
    rng: &mut Pokerng,
    rod: Wild3Action,
    lead: Gen3Lead,
    cycle_counter: &mut CycleCounter,
) -> usize {
    cycle_counter.on_moment_reached(Moment::ChooseWildMonIndex_Fishing);
    choose_index(rng, lead, rod, cycle_counter)
}

fn RockSmashWildEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Wild3GeneratorResults {
    cycle_counter.on_moment_reached(Moment::RockSmashWildEncounter);
    if !WildEncounterCheck(rng, map_data.rock_smash_rate, opts, cycle_counter) {
        return Wild3GeneratorResults::empty();
    }
    let Some((encounter_idx, level)) = TryGenerateWildMon(rng, opts, map_data, 0, cycle_counter)
    else {
        return Wild3GeneratorResults::empty();
    };
    finish(
        rng,
        opts,
        map_data,
        cycle_counter,
        encounter_idx,
        Some(level),
    )
}

fn WildEncounterCheck(
    rng: &mut Pokerng,
    encounter_rate: u32,
    opts: &Wild3GeneratorOptions,
    cycle_counter: &mut CycleCounter,
) -> bool {
    cycle_counter.on_moment_reached(Moment::WildEncounterCheck);
    let mut rate = encounter_rate * 16;
    if opts.using_white_flute {
        rate += rate / 2;
    }
    EncounterOddsCheck(rng, rate as u16, cycle_counter)
}

fn EncounterOddsCheck(
    rng: &mut Pokerng,
    encounter_rate: u16,
    cycle_counter: &mut CycleCounter,
) -> bool {
    cycle_counter.on_moment_reached(Moment::EncounterOddsCheck);
    rng.rand::<u16>() % 2880 < encounter_rate
}

fn TrySweetScentEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Wild3GeneratorResults {
    cycle_counter.on_moment_reached(Moment::TrySweetScentEncounter);
    SweetScentWildEncounter(rng, opts, map_data, cycle_counter)
}

fn SweetScentWildEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Wild3GeneratorResults {
    cycle_counter.on_moment_reached(Moment::SweetScentWildEncounter);
    if let Some(roamer) = TryStartRoamerEncounter(rng, opts, cycle_counter) {
        return finish(rng, opts, map_data, cycle_counter, roamer, None);
    }
    if opts.action == Wild3Action::SweetScentLand
        && DoMassOutbreakEncounterTest(rng, opts, cycle_counter)
    {
        let Some((outbreak, level)) =
            SetUpMassOutbreakEncounter(rng, 0, opts, map_data, cycle_counter)
        else {
            return Wild3GeneratorResults::empty();
        };
        return finish(rng, opts, map_data, cycle_counter, outbreak, Some(level));
    }
    let Some((encounter_idx, level)) = TryGenerateWildMon(rng, opts, map_data, 0, cycle_counter)
    else {
        return Wild3GeneratorResults::empty();
    };
    finish(
        rng,
        opts,
        map_data,
        cycle_counter,
        encounter_idx,
        Some(level),
    )
}

fn finish(
    rng: &Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &CycleCounter,
    encounter_idx: Wild3EncounterIndex,
    level: Option<u8>,
) -> Wild3GeneratorResults {
    generate_wild3_from_encounter(
        rng.clone(),
        opts,
        map_data,
        cycle_counter.clone(),
        encounter_idx,
        level,
    )
}

fn TryStartRoamerEncounter(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    cycle_counter: &mut CycleCounter,
) -> Option<Wild3EncounterIndex> {
    cycle_counter.on_moment_reached(Moment::TryStartRoamerEncounter);
    match opts.roamer_state {
        Wild3RoamerState::ActiveInMapLatias | Wild3RoamerState::ActiveInMapLatios
            if rng.rand::<u16>() % 4 == 0 =>
        {
            Some(Wild3EncounterIndex::Roamer(opts.roamer_state))
        }
        _ => None,
    }
}

fn DoMassOutbreakEncounterTest(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    cycle_counter: &mut CycleCounter,
) -> bool {
    cycle_counter.on_moment_reached(Moment::DoMassOutbreakEncounterTest);
    !matches!(
        opts.mass_outbreak_state,
        Wild3MassOutbreakState::Inactive | Wild3MassOutbreakState::ActiveNotInMap
    ) && rng.rand::<u16>() % 100 < 50
}

fn SetUpMassOutbreakEncounter(
    rng: &mut Pokerng,
    _flags: u8,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    cycle_counter: &mut CycleCounter,
) -> Option<(Wild3EncounterIndex, u8)> {
    cycle_counter.on_moment_reached(Moment::SetUpMassOutbreakEncounter);
    let index = Wild3EncounterIndex::MassOutbreak(opts.mass_outbreak_state);
    let encounter = map_data.get_encounter(opts.action, index)?;
    let level = ChooseWildMonLevel(rng, encounter, opts.lead, cycle_counter);
    CreateWildMon(encounter, level, cycle_counter);
    Some((index, level))
}

fn TryGenerateWildMon(
    rng: &mut Pokerng,
    opts: &Wild3GeneratorOptions,
    map_data: &Wild3MapGameData,
    _flags: u8,
    cycle_counter: &mut CycleCounter,
) -> Option<(Wild3EncounterIndex, u8)> {
    cycle_counter.on_moment_reached(Moment::TryGenerateWildMon);
    let slots = map_data.slots_by_action.get(opts.action as usize)?;
    let index = if opts.action == Wild3Action::SweetScentLand {
        TryGetAbilityInfluencedWildMonIndex(
            rng,
            slots,
            true,
            Gen3Lead::MagnetPull,
            opts.lead,
            cycle_counter,
        )
        .or_else(|| {
            TryGetAbilityInfluencedWildMonIndex(
                rng,
                slots,
                false,
                Gen3Lead::Static,
                opts.lead,
                cycle_counter,
            )
        })
        .unwrap_or_else(|| ChooseWildMonIndex_Land(rng, opts.lead, cycle_counter))
    } else {
        TryGetAbilityInfluencedWildMonIndex(
            rng,
            slots,
            false,
            Gen3Lead::Static,
            opts.lead,
            cycle_counter,
        )
        .unwrap_or_else(|| ChooseWildMonIndex_WaterRock(rng, opts.lead, cycle_counter))
    };
    let encounter = slots.get(index)?;
    let level = ChooseWildMonLevel(rng, encounter, opts.lead, cycle_counter);
    CreateWildMon(encounter, level, cycle_counter);
    Some((Wild3EncounterIndex::Slot((index as u8).into()), level))
}

fn TryGetAbilityInfluencedWildMonIndex(
    rng: &mut Pokerng,
    slots: &[Wild3EncounterGameData],
    steel: bool,
    ability: Gen3Lead,
    lead: Gen3Lead,
    cycle_counter: &mut CycleCounter,
) -> Option<usize> {
    cycle_counter.on_moment_reached(Moment::TryGetAbilityInfluencedWildMonIndex);
    if lead != ability || rng.rand::<u16>() % 2 != 0 {
        return None;
    }
    TryGetRandomWildMonIndexByType(rng, slots, steel, cycle_counter)
}

fn TryGetRandomWildMonIndexByType(
    rng: &mut Pokerng,
    slots: &[Wild3EncounterGameData],
    steel: bool,
    cycle_counter: &mut CycleCounter,
) -> Option<usize> {
    cycle_counter.on_moment_reached(Moment::TryGetRandomWildMonIndexByType);
    let matching: Vec<_> = slots
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| {
            let matched = if steel {
                slot.species_data.is_steel_type()
            } else {
                slot.species_data.is_electric_type()
            };
            matched.then_some(i)
        })
        .collect();
    if matching.is_empty() || matching.len() == slots.len() {
        return None;
    }
    Some(matching[rng.rand::<u16>() as usize % matching.len()])
}

fn choose_index(
    rng: &mut Pokerng,
    lead: Gen3Lead,
    action: Wild3Action,
    cycle_counter: &mut CycleCounter,
) -> usize {
    match lead {
        Gen3Lead::Egg => cycle_counter.add_cycle(2819),
        _ => cycle_counter.add(12059, 32),
    }
    let moment = if action == Wild3Action::SweetScentLand {
        Moment::ChooseWildMonIndex_Land_Random
    } else {
        Moment::ChooseWildMonIndex_WaterRock_Random
    };
    cycle_counter.on_moment_reached(moment);
    let value = rng.rand::<u16>() as u32;
    cycle_counter.add_cycle(if lead == Gen3Lead::Egg { 234 } else { 378 });
    cycle_counter.add_cycle(calc_modulo_cycle_unsigned(value, 100));
    let slot =
        EncounterSlot::from_rand((value % 100) as u8, EncounterSlot::gen3_thresholds(action));
    slot as usize
}

fn ChooseWildMonIndex_Land(
    rng: &mut Pokerng,
    lead: Gen3Lead,
    cycle_counter: &mut CycleCounter,
) -> usize {
    cycle_counter.on_moment_reached(Moment::ChooseWildMonIndex_Land);
    choose_index(rng, lead, Wild3Action::SweetScentLand, cycle_counter)
}

fn ChooseWildMonIndex_WaterRock(
    rng: &mut Pokerng,
    lead: Gen3Lead,
    cycle_counter: &mut CycleCounter,
) -> usize {
    cycle_counter.on_moment_reached(Moment::ChooseWildMonIndex_WaterRock);
    choose_index(rng, lead, Wild3Action::SweetScentWater, cycle_counter)
}

fn ChooseWildMonLevel(
    rng: &mut Pokerng,
    encounter: &Wild3EncounterGameData,
    lead: Gen3Lead,
    cycle_counter: &mut CycleCounter,
) -> u8 {
    cycle_counter.on_moment_reached(Moment::ChooseWildMonLevel);
    cycle_counter.on_moment_reached(Moment::ChooseWildMonLevel_RandomLvl);
    let range = encounter.max_level - encounter.min_level + 1;
    let value = rng.rand::<u16>();
    cycle_counter.add_cycle(calc_modulo_cycle_signed(value as i32, range as i32));
    let mut increment = (value % range as u16) as u8;
    if lead == Gen3Lead::HustleVitalSpiritPressure {
        if rng.rand::<u16>() % 2 == 0 {
            return encounter.max_level;
        }
        increment = increment.saturating_sub(1);
    }
    encounter.min_level + increment
}

fn CreateWildMon(
    _encounter: &Wild3EncounterGameData,
    _level: u8,
    cycle_counter: &mut CycleCounter,
) {
    cycle_counter.on_moment_reached(Moment::CreateWildMon);
    // PID and IV generation continues in the shared generator.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Species,
        gen3::{Gen3Method, Wild3SpecialEncounterGameData, generate_gen3_wild},
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
                    let old = generate_gen3_wild(Pokerng::new(seed), &opts, &map);
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
                    let old = generate_gen3_wild(Pokerng::new(seed), &opts, &map);
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
}
