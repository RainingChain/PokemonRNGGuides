import React from "react";
import { z } from "zod";
import {
  RngToolForm,
  Field,
  Flex,
  ResultColumn,
  Icon,
  FormFieldTable,
  FormikRadio,
} from "~/components";
import { FormikSelect } from "~/components/select";
import { RngToolSubmit } from "~/components/rngToolForm";
import { Typography } from "~/components/typography";
import { nature } from "~/types/nature";
import { Button } from "~/components/button";
import { toOptions } from "~/utils/options";
import {
  getPkmFilterInitialValues,
  natureOptions,
  pkmFilterFieldsToRustInput,
} from "~/components/pkmFilter";
import { getStatFields } from "~/rngToolsUi/shared/statFields";
import { gen3Methods, getLooseBaseStats, StatFieldsSchema } from "~/types";
import {
  Gen3Method,
  rngTools,
  StatsValue,
  Wild3EncounterGameData,
  Wild3MapSetups,
  Wild3SearcherOptions,
  Wild3SearcherResultMon,
} from "~/rngTools";
import { getWild3EmeraldGameData } from "./data/wild3GameData";
import type { FormState as TargetSetup } from "./wild3CalibTarget";
import { gen3Leads, isFishingAction, wild3Actions } from "./utils";
import { useWatch } from "react-hook-form";
import { GenderFilter } from "~/components/genderFilter";
import { getStatRange } from "~/types/statRange";
import uniq from "lodash-es/uniq";
import {
  gen3PkmFilterFieldsToRustInput,
  getGen3PkmFilterInitialValues,
} from "~/components/gen3PkmFilter";
import clamp from "lodash-es/clamp";
import { Tooltip } from "antd";
import { formatProbability } from "~/utils/formatProbability";

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
  methods: Gen3Method[];
  score: number;
  probabilityHitMethodsAtAdvance: number;
};

const CONFIDENCE_RANGE = 10_000; // we assume the player hits its target advance by more or less 10K (~2m45s)

export const searchCaughtMon = async (
  values: FormState,
  targetSetup: TargetSetup,
) => {
  const initial_seed = targetSetup.usingPaintingReseeding
    ? targetSetup.targetPaintingSeed
    : 0;

  const initial_advances = Math.max(
    targetSetup.targetAdvance - CONFIDENCE_RANGE,
    0,
  );

  const map_data = emeraldWildGameData.maps_data.find(
    (map) => map.map_id === targetSetup.map,
  );
  if (map_data == null) {
    return [];
  }
  const map_setup: Wild3MapSetups = {
    map_data,
    actions: [targetSetup.action],
    roamer_states: [targetSetup.roamerState],
    mass_outbreak_states: [targetSetup.massOutbreakState],
    feebas_states: [targetSetup.feebasState],
  };

  const caughtStats: StatsValue = {
    hp: values.hpStat,
    atk: values.atkStat,
    def: values.defStat,
    spa: values.spaStat,
    spd: values.spdStat,
    spe: values.speStat,
  };

  const baseStats = getLooseBaseStats(values.species);
  if (baseStats == null) {
    return [];
  }

  console.log(caughtStats);

  const opts: Wild3SearcherOptions = {
    initial_seed,
    tid: 0, // doesn't matter
    sid: 0, // doesn't matter
    initial_advances,
    max_advances: CONFIDENCE_RANGE * 2,
    max_result_count: 2 ** 32 - 1, // No limit
    filter: pkmFilterFieldsToRustInput({
      ...getPkmFilterInitialValues(),
      filter_nature: values.nature,
      filter_gender: values.filter_gender,
      filter_stats: {
        lvl: values.lvl,
        base_stats: baseStats,
        min_stats: caughtStats,
        max_stats: caughtStats,
      },
    }),
    gen3_filter: gen3PkmFilterFieldsToRustInput(
      {
        ...getGen3PkmFilterInitialValues(),
        filter_lvl: values.lvl,
      },
      values.species,
    ),
    leads: [gen3Leads[targetSetup.leadIdx]],
    map_setups: [map_setup],
    methods: gen3Methods,
    consider_cycles: true,
    consider_rng_manipulated_lead_pid: true,
    generate_even_if_impossible: true,
    painting_opts: null,
  };

  const resultsByPidPath = await rngTools.search_wild3(opts);
  const results = resultsByPidPath.map((pidPath) => pidPath.vec).flat();
  const resultsByAdv = new Map<number, Wild3SearcherResultMon[]>();
  results.forEach((res) => {
    const list = resultsByAdv.get(res.advance);
    if (list != null) {
      list.push(res);
    } else {
      resultsByAdv.set(res.advance, [res]);
    }
  });

  const list = Array.from(resultsByAdv.entries()).map(([adv, results]) => {
    // TODO: consider the actual lead pid speed by running method_distribution for each results.
    // right now, we assume common_upper_lead cycle speed (868)
    const probabilityHitMethodsAtAdvance = results.reduce((prev, res) => {
      return (
        prev +
        (res.cycle_data_by_lead?.common_upper_lead.method_probability ?? 0)
      );
    }, 0);
    const scoreHitMethodsAtAdvance = clamp(
      probabilityHitMethodsAtAdvance,
      0.01,
      1,
    );

    const distanceFromTarget = Math.abs(targetSetup.targetAdvance - adv);

    const score = -distanceFromTarget * (1 - scoreHitMethodsAtAdvance);

    return {
      advance: adv,
      targetAdvance: targetSetup.targetAdvance,
      methods: uniq(results.map((res) => res.method)).toSorted(),
      score,
      probabilityHitMethodsAtAdvance,
    };
  });

  return list.sort((res1, res2) => {
    return res2.score - res1.score;
  });
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
  const selectedLvl = useWatch<FormState, "lvl">({ name: "lvl" });
  const selectedNature = useWatch<FormState, "nature">({ name: "nature" });

  const [fields, setFields] = React.useState<Field[]>([]);

  React.useEffect(() => {
    const encounters = getPossibleEncountersForMap(targetSetup);
    const speciesList = uniq(
      encounters.map((enc) => enc.species_data.species),
    ).toSorted();
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
      setFields([speciesField]);
      return;
    }

    const genderRatio = selectedSpeciesInfos[0].species_data.gender_ratio;
    const lvls = new Set<number>();
    selectedSpeciesInfos.forEach((info) => {
      for (let lvl = info.min_level; lvl <= info.max_level; lvl++) {
        lvls.add(lvl);
      }
    });
    const sortedLvls = Array.from(lvls).sort((lvl1, lvl2) => lvl1 - lvl2);

    getStatRange(
      selectedSpecies,
      [selectedLvl, selectedLvl],
      selectedNature,
    ).then((minMaxStats) => {
      setFields([
        speciesField,
        {
          label: "Level",
          input: (
            <FormikRadio<FormState>
              name="lvl"
              options={toOptions(sortedLvls)}
            />
          ),
        },
        {
          label: "Gender",
          input: <GenderFilter genderRatio={genderRatio} permitAny={false} />,
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
      ]);
    });
  }, [targetSetup, selectedSpecies, selectedLvl, selectedNature]);

  return <FormFieldTable fields={fields} />;
};

export const Wild3CalibCaughtMon = ({
  targetSetup,
  setLatestHitAdv,
}: Props) => {
  const [results, setResults] = React.useState<CaughtMonResult[]>([]);
  const { targetMethod, targetAdvance } = targetSetup;

  const onSubmit = React.useCallback<RngToolSubmit<FormState>>(
    async (values) => {
      setResults(await searchCaughtMon(values, targetSetup));
    },
    [setResults, targetSetup],
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
        title: "Methods",
        dataIndex: "methods",
        render(methods, values) {
          const title = `${formatProbability(values.probabilityHitMethodsAtAdvance)} likelihood if the hit advance is ${values.advance}`;
          return <Tooltip title={title}>{methods.join(", ")}</Tooltip>;
        },
      },
      {
        title: "",
        dataIndex: "advance",
        render(advance, values) {
          if (
            values.advance === targetAdvance &&
            values.methods.includes(targetMethod)
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
          <Fields targetSetup={targetSetup} />
        </div>
      </RngToolForm>
    </Flex>
  );
};
