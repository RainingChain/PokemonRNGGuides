import { z } from "zod";
import { RngToolForm, Field, RngToolSubmit } from "~/components";
import { Game } from "./index";
import { findTargetAdvanceForShinyPokemon } from "./calc";

const Validator = z.object({
  tid: z.number().int().min(0).max(65535),
  sid: z.number().int().min(0).max(65535),
});

export type FormState = z.infer<typeof Validator>;

const initialValues: FormState = {
  tid: 0,
  sid: 0,
};

type Props = {
  game: Game;
  setTargetAdvance: (targetAdvance: number) => void;
  usingDeadBattery: boolean;
  initialTidSid?: FormState;
};

export const FindTargetAdvance = ({
  game,
  setTargetAdvance,
  usingDeadBattery,
  initialTidSid = initialValues,
}: Props) => {
  const fields: Field[] = [
    { label: "TID", input: <>{initialTidSid.tid}</> },
    { label: "SID", input: <>{initialTidSid.sid}</> },
  ];

  const onSubmit: RngToolSubmit<FormState> = async (opts) => {
    const targetAdvance = await findTargetAdvanceForShinyPokemon(
      game,
      opts.tid,
      opts.sid,
      usingDeadBattery,
    );
    if (targetAdvance !== null) {
      setTargetAdvance(targetAdvance);
    }
  };

  return (
    <RngToolForm<FormState, never[]>
      formContainerId="find-target-advance"
      fields={fields}
      initialValues={initialTidSid}
      validationSchema={Validator}
      submitTrackerId="findTarget"
      submitButtonLabel="Calculate Target advance"
      onSubmit={onSubmit}
    />
  );
};
