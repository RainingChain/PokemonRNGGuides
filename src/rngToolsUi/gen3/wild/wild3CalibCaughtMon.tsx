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
import {
  Gen3Method,
  Species,
  SpeciesData,
  Wild3Action,
  Wild3EncounterGameData,
} from "~/rngTools";
import { getWild3EmeraldGameData } from "./data/wild3GameData";
import type { FormState as TargetSetup } from "./wild3CalibTarget";
import { isFishingAction, wild3Actions } from "./utils";
import { useWatch } from "react-hook-form";
import { GenderFilter } from "~/components/genderFilter";

const emeraldWildGameData = getWild3EmeraldGameData();

const Validator = z
  .object({
    nature: z.enum(nature),
    filter_gender: z.enum(["Male", "Female"]),
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
  filter_gender: "Male", // Must be named filter_gender for GenderFilter component
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

const getPossibleEncountersForMap = (targetSetup: TargetSetup) => {
  const map_data = emeraldWildGameData.maps_data.find(
    (map) => map.map_id === targetSetup.map,
  );
  if (map_data == null) {
    return [];
  }

  const list: Wild3EncounterGameData[] = [
    ...map_data.slots_by_action[wild3Actions.indexOf(targetSetup.action)],
  ];
  if (isFishingAction(targetSetup.action) && map_data.feebas != null) {
    list.push(map_data.feebas);
  }
  if (targetSetup.action === "SweetScentLand") {
    list.push(
      ...map_data.roamers
        .filter((roamer) => roamer.id === targetSetup.roamerState)
        .map((roamer) => roamer.encounter_data),
    );
  }
  if (
    targetSetup.action === "SweetScentLand" ||
    targetSetup.action === "SweetScentWater"
  ) {
    list.push(
      ...map_data.mass_outbreaks
        .filter(
          (mass_outbreak) => mass_outbreak.id === targetSetup.massOutbreakState,
        )
        .map((mass_outbreak) => mass_outbreak.encounter_data),
    );
  }
  return list;
};

const Fields = ({ targetSetup }: { targetSetup: TargetSetup }) => {
  const selectedSpecies = useWatch<FormState, "species">({ name: "species" });

  const fields = React.useMemo(() => {
    const encounters = getPossibleEncountersForMap(targetSetup);
    const speciesList = encounters.map((enc) => enc.species_data.species);
    const selectedSpeciesInfos = encounters.filter(
      (enc) => enc.species_data.species === selectedSpecies,
    );

    const speciesField: Field = {
      label: "Species",
      input: (
        <FormikRadio<FormState>
          name="species"
          options={toOptions(speciesList)}
        />
      ),
    };

    if (selectedSpeciesInfos.length === 0) {
      return [];
    }

    const genderRatio = selectedSpeciesInfos?.[0].species_data.gender_ratio;
    const minMaxLvl = Math.min(
      ...selectedSpeciesInfos.map((info) => info.min_level),
    );

    return [
      {
        label: "Species",
        input: (
          <FormikRadio<FormState>
            name="species"
            options={toOptions(speciesList)}
          />
        ),
      },
      {
        //TODO
        label: "Level",
        input: (
          <FormikRadio<FormState>
            name="lvl"
            options={toOptions(["Male", "Female"] as const)}
          />
        ),
      },
      {
        label: "Gender",
        input: <GenderFilter genderRatio={genderRatio} />,
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
      ...getStatFields<FormState>(defaultMinMaxStats), //NO_PROD init defaultMinMaxStats with species
    ];
  }, [targetSetup]);

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
        <div style={{ paddingLeft: "20px" }}>
          <Fields />
        </div>
      </RngToolForm>
    </Flex>
  );
};
