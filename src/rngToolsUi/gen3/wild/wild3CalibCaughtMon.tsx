import React from "react";
import { z } from "zod";
import {
  RngToolForm,
  Field,
  Flex,
  ResultColumn,
  Icon,
  FormFieldTable,
} from "~/components";
import { FormikRadio } from "~/components/radio";
import { FormikSelect } from "~/components/select";
import { RngToolSubmit } from "~/components/rngToolForm";
import { Typography } from "~/components/typography";
import { nature } from "~/types/nature";
import { Button } from "~/components/button";
import { toOptions } from "~/utils/options";
import { natureOptions } from "~/components/pkmFilter";
import { getStatFields } from "~/rngToolsUi/shared/statFields";
import { defaultMinMaxStats, StatFieldsSchema } from "~/types";
import { Gen3Method, Species, Wild3EncounterGameData } from "~/rngTools";
import { getWild3EmeraldGameData } from "./data/wild3GameData";
import type { FormState as TargetSetup } from "./wild3CalibTarget";

const emeraldWildGameData = getWild3EmeraldGameData();

const Validator = z
  .object({
    nature: z.enum(nature),
    gender: z.enum(["Male", "Female"]),
    species: z.enum(emeraldWildGameData.species),
    lvl: z.number().min(1).max(100),
    //NO_PROD ability
  })
  .extend(StatFieldsSchema.shape);

export type FormState = z.infer<typeof Validator>;

const initialValues: FormState = {
  hpStat: 0,
  atkStat: 0,
  defStat: 0,
  spaStat: 0,
  spdStat: 0,
  speStat: 0,
  nature: "Adamant",
  gender: "Male",
  species: "Shuckle",
  lvl: 1,
};

type Props = {
  targetSetup: TargetSetup;
  setLatestHitAdv: (hitAdv: number) => void;
};

export type CaughtMonResult = {
  advance: number;
  targetAdvance: number;
  method: Gen3Method;
};

const Fields = () => {
  //NO_PROD species first. if gender if not genderless etc.
  const speciesList = []; // based on targetSetup
  const fields = React.useMemo(
    () => [
      {
        label: "Species",
        input: (
          <FormikRadio<FormState>
            name="species"
            options={toOptions(speciesList)}
          />
        ),
        indent: 1,
      },
      /* { //TODO
        label: "Level",
        input: (
          <FormikRadio<FormState>
            name="gender"
            options={toOptions(["Male", "Female"] as const)}
          />
        ),
      },*/
      {
        label: "Gender",
        input: (
          <FormikRadio<FormState>
            name="gender"
            options={toOptions(["Male", "Female"] as const)}
          />
        ),
        indent: 1,
      },
      {
        label: "Nature",
        input: (
          <FormikSelect<FormState, "nature">
            name="nature"
            options={natureOptions.required}
          />
        ),
        indent: 1,
      },
      ...getStatFields<FormState>(defaultMinMaxStats), //NO_PROD init defaultMinMaxStats with species
    ],
    [],
  );

  return <FormFieldTable fields={fields} />;
};

export const Wild3CalibCaughtMon = ({
  targetSetup,
  setLatestHitAdv,
}: Props) => {
  const [results, setResults] = React.useState<CaughtMonResult[]>([]);
  const { targetMethod, targetAdvance } = targetSetup;

  const onSubmit = React.useCallback<RngToolSubmit<FormState>>(
    async (opts) => {
      //NO_PROD
      /*setResults(
        await generateCaughtMonResults(
          game,
          targetAdvance,
          targetStarter,
          opts,
        ),
      );*/
    },
    [targetAdvance, setResults],
  );

  const columns = React.useMemo((): ResultColumn<CaughtMonResult>[] => {
    const columns: ResultColumn<CaughtMonResult>[] = [
      { title: "Target", dataIndex: "targetAdvance" },
      {
        title: "Advance",
        dataIndex: "advance",
        render: (val, values) => {
          const diffWithTarget = val - values.targetAdvance;
          if (diffWithTarget === 0) {
            return `${val}`;
          }
          if (diffWithTarget > 0) {
            return `${val} (+${diffWithTarget})`;
          }
          return `${val} (${diffWithTarget})`;
        },
      },
      {
        title: "Method",
        dataIndex: "method",
      },
      {
        title: "",
        dataIndex: "advance",
        render(advance, values) {
          if (
            values.advance === targetAdvance &&
            values.method === targetMethod
          ) {
            return "Target Pokémon";
          }

          return (
            <Button
              type="text"
              color="PrimaryText"
              trackerId="wild3CalibCaughtMon_adv"
              onClick={() => {
                setLatestHitAdv(advance);
                setResults([]);
              }}
            >
              <Icon name="Update" size={20} /> Update Calibration
            </Button>
          );
        },
      },
    ];
    return columns;
  }, [setLatestHitAdv, setResults, targetMethod, targetAdvance]);
  /*
  export const getStatRange = async (
    species: Species,
    levelRange: [number, number] = [5, 5],
  ):*/

  return (
    <Flex vertical gap={8}>
      <Typography.Title level={5} p={0} m={0}>
        Caught Pokémon
      </Typography.Title>
      <RngToolForm<FormState, CaughtMonResult>
        formContainerId="generate-wild3-caught"
        columns={columns}
        results={results}
        initialValues={initialValues}
        validationSchema={Validator}
        onSubmit={onSubmit}
        submitTrackerId="generate_wild3_caught"
        submitButtonLabel="Find advances matching caught Pokémon"
        rowKey="advance"
      >
        <Fields />
      </RngToolForm>
    </Flex>
  );
};
