use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::gen3::{
    BASE_LEAD_PID_MOD_24_CYCLES, COMMON_LEAD_RANGE, CycleAndModCount, CycleAndModRange,
    CycleCounter, FASTEST_MODULO_CYCLE_24, Gen3Lead, INFINITE_CYCLE, Moment,
    SLOWEST_MODULO_CYCLE_24, VBLANK_FREQ, Wild3Action, Wild3GeneratorOptions,
    get_min_mid_max_pre_sweet_scent_cycle, is_method_possible_to_trigger,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum CycleFrameCounterMode {
    #[default]
    Inactive,
    MinMaxRange,
    DetailedBreakdown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CycleFrame {
    pub cycle: usize,
    pub frame: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CycleFrameMoment {
    pub cycle: usize,
    pub frame: usize,
    pub moment: Moment,
}

impl CycleFrame {
    pub fn add_cycle(&mut self, cycle: usize) {
        self.cycle += cycle;
        while self.cycle > VBLANK_FREQ {
            self.cycle -= VBLANK_FREQ;
            self.frame += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MinMaxCycleFrame {
    // assumes min_lead_cycle_spd
    pub min_cycle: CycleFrame,
    // assumes max_lead_cycle_spd
    pub max_cycle: CycleFrame,
    pub min_lead_cycle_spd: usize,
    pub max_lead_cycle_spd: usize,
}

impl MinMaxCycleFrame {
    pub fn new_inactive() -> Self {
        Self {
            min_cycle: CycleFrame { cycle: 0, frame: 0 },
            max_cycle: CycleFrame { cycle: 0, frame: 0 },
            min_lead_cycle_spd: 0,
            max_lead_cycle_spd: 0,
        }
    }

    pub fn new(
        is_egg_lead: bool,
        action: Wild3Action,
        consider_rng_manipulated_lead_pid: bool,
    ) -> Self {
        let (min_cycle, _, max_cycle) = get_min_mid_max_pre_sweet_scent_cycle(action);
        Self {
            min_cycle: CycleFrame {
                cycle: min_cycle,
                frame: 0,
            },
            max_cycle: CycleFrame {
                cycle: max_cycle,
                frame: 0,
            },
            min_lead_cycle_spd: if is_egg_lead {
                0
            } else if consider_rng_manipulated_lead_pid {
                FASTEST_MODULO_CYCLE_24
            } else {
                COMMON_LEAD_RANGE.start
            },
            max_lead_cycle_spd: if is_egg_lead {
                0
            } else if consider_rng_manipulated_lead_pid {
                SLOWEST_MODULO_CYCLE_24
            } else {
                COMMON_LEAD_RANGE.end
            },
        }
    }
    pub fn add_cycle(&mut self, cycle: usize) {
        self.min_cycle.add_cycle(cycle);
        self.max_cycle.add_cycle(cycle);
    }
    pub fn add_mod(&mut self, lead_pid_mod: usize) {
        self.min_cycle
            .add_cycle(lead_pid_mod * self.min_lead_cycle_spd);
        self.max_cycle
            .add_cycle(lead_pid_mod * self.max_lead_cycle_spd);
    }
    pub fn can_vblank_occur_soon(&self, cycle_range: usize) -> bool {
        if self.max_cycle.frame > self.min_cycle.frame {
            return true;
        }
        self.max_cycle.cycle > VBLANK_FREQ - cycle_range
    }
}

impl Default for MinMaxCycleFrame {
    fn default() -> Self {
        MinMaxCycleFrame {
            min_cycle: CycleFrame {
                cycle: 30_000, //NO_PROD
                frame: 0,
            },
            max_cycle: CycleFrame {
                cycle: 80_000, //NO_PROD
                frame: 0,
            },
            min_lead_cycle_spd: 0,
            max_lead_cycle_spd: 900,
        }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum CycleFrameCounter {
    Inactive,
    MinMaxRange {
        min_max_cycles: MinMaxCycleFrame,
        cycle_instability: f32,
        base_cycle_count: usize,
        lead_pid_mod_count: usize,
    },
}

impl CycleFrameCounter {
    pub fn new_inactive() -> Self {
        CycleFrameCounter::Inactive
    }
    pub fn new_for_min_max_range(
        is_egg_lead: bool,
        action: Wild3Action,
        consider_rng_manipulated_lead_pid: bool,
    ) -> Self {
        CycleFrameCounter::MinMaxRange {
            min_max_cycles: MinMaxCycleFrame::new(
                is_egg_lead,
                action,
                consider_rng_manipulated_lead_pid,
            ),
            cycle_instability: 0.0,
            base_cycle_count: 0,
            lead_pid_mod_count: 0,
        }
    }
    pub fn add(&mut self, cycle: usize, lead_pid_mod: usize) {
        match self {
            CycleFrameCounter::Inactive => {}
            CycleFrameCounter::MinMaxRange { .. } => {
                self.add_cycle(cycle);
                self.add_mod(lead_pid_mod);
            }
        }
    }
    pub fn add_cycle(&mut self, cycle: usize) {
        match self {
            CycleFrameCounter::Inactive => {}
            CycleFrameCounter::MinMaxRange {
                base_cycle_count,
                min_max_cycles,
                ..
            } => {
                *base_cycle_count += cycle;
                min_max_cycles.add_cycle(cycle);
            }
        }
    }
    pub fn add_mod(&mut self, lead_pid_mod: usize) {
        match self {
            CycleFrameCounter::Inactive => {}
            CycleFrameCounter::MinMaxRange {
                base_cycle_count,
                lead_pid_mod_count,
                min_max_cycles,
                ..
            } => {
                *base_cycle_count += lead_pid_mod * BASE_LEAD_PID_MOD_24_CYCLES;
                *lead_pid_mod_count += lead_pid_mod;
                min_max_cycles.add_mod(lead_pid_mod);
            }
        }
    }
    pub fn on_moment_reached(&mut self, _moment: Moment) {
        /*if self.mode != CycleFrameCounterMode::DetailedBreakdown {
            return;
        }
        self.cycle_at_moments.push(CycleAndModAtMoment {
            cycle: self.cycle.cycle,
            lead_pid_mod: self.cycle.lead_pid_mod,
            moment,
        });*/
    }
    /** can a vblank occurs between now and in cycle_range */
    pub fn can_vblank_occur_soon(&self, cycle_range: usize) -> bool {
        match self {
            CycleFrameCounter::Inactive => true,
            CycleFrameCounter::MinMaxRange { min_max_cycles, .. } => {
                min_max_cycles.can_vblank_occur_soon(cycle_range)
            }
        }
    }
    pub fn is_possible_that_no_vblank_yet(&self) -> bool {
        match self {
            CycleFrameCounter::Inactive => true,
            CycleFrameCounter::MinMaxRange { min_max_cycles, .. } => {
                min_max_cycles.min_cycle.frame == 0
            }
        }
    }
    pub fn can_generate_method(&self, opts: &Wild3GeneratorOptions, len: usize) -> bool {
        if opts.generate_even_if_impossible {
            return true;
        }

        match self {
            CycleFrameCounter::Inactive => true,
            CycleFrameCounter::MinMaxRange { .. } => {
                if !opts.consider_rng_manipulated_lead_pid {
                    return is_method_possible_to_trigger(
                        &self.create_cycle_range(len),
                        opts.action,
                        opts.lead == Gen3Lead::Egg,
                        false,
                        opts.lead_cycle_speed,
                    );
                }
                if len == INFINITE_CYCLE {
                    self.is_possible_that_no_vblank_yet()
                } else {
                    self.can_vblank_occur_soon(len)
                }
            }
        }
    }
    pub fn create_cycle_range(&self, len: usize) -> CycleAndModRange {
        match self {
            CycleFrameCounter::Inactive => CycleAndModRange::new(0, 0, 0),
            CycleFrameCounter::MinMaxRange {
                lead_pid_mod_count,
                base_cycle_count,
                ..
            } => CycleAndModRange::new(*base_cycle_count, *lead_pid_mod_count, len),
        }
    }
    pub fn to_cycle_counter(&self) -> CycleCounter {
        match self {
            CycleFrameCounter::Inactive => CycleCounter::default(),
            CycleFrameCounter::MinMaxRange {
                base_cycle_count,
                lead_pid_mod_count,
                cycle_instability,
                ..
            } => CycleCounter {
                cycle: CycleAndModCount {
                    cycle: *base_cycle_count,
                    lead_pid_mod: *lead_pid_mod_count,
                },
                cycle_instability: *cycle_instability,
                ..Default::default()
            },
        }
    }

    pub fn get_current_cycle_count(&self) -> usize {
        match self {
            CycleFrameCounter::Inactive => 0,
            CycleFrameCounter::MinMaxRange {
                base_cycle_count, ..
            } => *base_cycle_count,
        }
    }
    pub fn set_cycle_instability(&mut self, instability: f32) {
        match self {
            CycleFrameCounter::Inactive => {}
            CycleFrameCounter::MinMaxRange {
                cycle_instability, ..
            } => *cycle_instability = instability,
        }
    }
}
