import React from "react";
import { z } from "zod";
import { RngToolForm, Field, Flex, ResultColumn, Icon } from "~/components";
import { FormikRadio } from "~/components/radio";
import { FormikSelect } from "~/components/select";
import { RngToolSubmit } from "~/components/rngToolForm";
import { Typography } from "~/components/typography";
import { nature } from "~/types/nature";
import { Button } from "~/components/button";
import { toOptions } from "~/utils/options";
import { natureOptions } from "~/components/pkmFilter";
import { getStatFields } from "~/rngToolsUi/shared/statFields";
import { StatFieldsSchema } from "~/types";
import { Species, Wild3EncounterGameData } from "~/rngTools";
import { getWild3EmeraldGameData } from "./data/wild3GameData";

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
  possibleEncounters: Wild3EncounterGameData[];
  targetAdvance: number;
};

export type CaughtMonResult = {
  advance: number;
  targetAdvance: number;
};

const getFields = (): Field[] => {
  //NO_PROD species first. if gender if not genderless etc.
  return [
    {
      label: "Gender",
      input: (
        <FormikRadio<FormState>
          name="gender"
          options={toOptions(["Male", "Female"] as const)}
        />
      ),
    },
    {
      label: "Nature",
      input: (
        <FormikSelect<FormState, "nature">
          name="nature"
          options={natureOptions.required}
        />
      ),
    },
    ...getStatFields<FormState>(minMaxStats),
  ];
};

export const CaughtMon = ({ possibleEncounters, targetAdvance }: Props) => {
  const [results, setResults] = React.useState<CaughtMonResult[]>([]);

  const onSubmit = React.useCallback<RngToolSubmit<FormState>>(
    async (opts) => {
      setResults(
        await generateCaughtMonResults(
          game,
          targetAdvance,
          targetStarter,
          opts,
        ),
      );
    },
    [game, targetAdvance, targetStarter, setResults],
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
        title: "",
        dataIndex: "advance",
        render(advance, values) {
          if (values.advance === values.targetAdvance) {
            return "Shiny if correct SID";
          }

          return (
            <Button
              type="text"
              color="PrimaryText"
              trackerId="wild3CalibCaughtMon_adv"
              onClick={() => {
                //setLatestHitAdv(advance); //NO_PROD
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
  }, [setLatestHitAdv, setResults]);
  /*
  export const getStatRange = async (
    species: Species,
    levelRange: [number, number] = [5, 5],
  ):*/

  const fields = React.useMemo((): Field[] => {
    return [
      {
        label: "Gender",
        input: (
          <FormikRadio<FormState>
            name="gender"
            options={toOptions(["Male", "Female"] as const)}
          />
        ),
      },
      {
        label: "Nature",
        input: (
          <FormikSelect<FormState, "nature">
            name="nature"
            options={natureOptions.required}
          />
        ),
      },
      ...getStatFields<FormState>(minMaxStats),
    ];
  }, [minMaxStats]);

  return (
    <Flex vertical gap={8}>
      <Typography.Title level={5} p={0} m={0}>
        Caught Pokémon
      </Typography.Title>
      <RngToolForm<FormState, CaughtMonResult>
        formContainerId="generate-gen3-caught-starter"
        fields={fields}
        columns={columns}
        results={results}
        initialValues={initialValues}
        validationSchema={Validator}
        onSubmit={onSubmit}
        submitTrackerId="generate_gen3_caught_starter"
        submitButtonLabel="Find advances matching caught starter Pokémon"
        rowKey="advance"
      />
    </Flex>
  );
};
