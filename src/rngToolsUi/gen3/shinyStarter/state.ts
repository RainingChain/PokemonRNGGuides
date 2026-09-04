import { atom } from "jotai";
import { Gen3NearbySid } from "~/rngTools";
import { Gen3Console } from "~/types/console";

export const usingDeadBatteryAtom = atom(true);
export const shinyStarterConsoleAtom = atom<Gen3Console>("GBA");

export type ShinyStarterTidSidResult = Gen3NearbySid & {
  tid: number;
  tid_gen_target_adv: number;
  crossedOut: boolean;
};

export const shinyStarterTidSidResultsAtom = atom<ShinyStarterTidSidResult[]>(
  [],
);
