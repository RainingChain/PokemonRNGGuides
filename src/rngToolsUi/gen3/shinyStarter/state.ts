import { atom } from "jotai";
import { Gen3NearbySid } from "~/rngTools";

export const usingDeadBatteryAtom = atom(true);

export type ShinyStarterTidSidResult = Gen3NearbySid & {
  tid: number;
  tid_gen_target_adv: number;
  crossedOut: boolean;
};

export const shinyStarterTidSidResultsAtom = atom<ShinyStarterTidSidResult[]>(
  [],
);
