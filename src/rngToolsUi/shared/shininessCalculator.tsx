import React from "react";
import {
  Flex,
  FormFieldTable,
  NumberInput,
  Typography,
  Field,
} from "~/components";

const MAX_U16 = 0xffff;
const MAX_U32 = 0xffffffff;

const clamp = (value: number | null, max: number) => {
  if (value == null || value < 0) {
    return 0;
  }

  return Math.min(value, max);
};

const pidHigh = (pid: number) => (pid >>> 16) & MAX_U16;
const pidLow = (pid: number) => pid & MAX_U16;
const shinyValue = (tid: number, sid: number, pid: number) =>
  (tid ^ sid ^ pidHigh(pid) ^ pidLow(pid)) & MAX_U16;
const tsv = (tid: number, sid: number) => (tid ^ sid) >>> 3;
const psv = (pid: number) => (pidHigh(pid) ^ pidLow(pid)) >>> 3;
const findSquareShinySid = (tid: number, pid: number) =>
  (tid ^ pidHigh(pid) ^ pidLow(pid)) & MAX_U16;

const formatId = (value: number) => value.toString();

const CalculatorDescription = ({ children }: { children: React.ReactNode }) => (
  <Typography.Text>{children}</Typography.Text>
);

const OutputValue = ({ children }: { children: React.ReactNode }) => (
  <Typography.Text strong>{children}</Typography.Text>
);

export const FindSidForShinyPokemon = () => {
  const [tid, setTid] = React.useState(0);
  const [pid, setPid] = React.useState(0);

  const sid = findSquareShinySid(tid, pid);
  const targetTsv = tsv(tid, sid);
  const targetPsv = psv(pid);

  const fields: Field[] = [
    {
      label: "TID",
      input: (
        <NumberInput
          numType="decimal"
          value={tid}
          onChange={(value) => setTid(clamp(value, MAX_U16))}
        />
      ),
    },
    {
      label: "PID",
      input: (
        <NumberInput
          numType="hex"
          value={pid}
          onChange={(value) => setPid(clamp(value, MAX_U32))}
        />
      ),
    },
    {
      label: "SID",
      input: <OutputValue>{formatId(sid)}</OutputValue>,
    },
    {
      label: "TSV",
      input: <OutputValue>{formatId(targetTsv)}</OutputValue>,
    },
    {
      label: "PSV",
      input: <OutputValue>{formatId(targetPsv)}</OutputValue>,
    },
  ];

  return (
    <Flex vertical gap={16}>
      <Flex vertical gap={4}>
        <Typography.Title level={5} mv={0}>
          Find SID for Shiny Pokemon
        </Typography.Title>
        <CalculatorDescription>
          Find SID to make specific target Pokemon shiny.{" "}
          <a href="TODO link">Change your SID with ACE</a>
        </CalculatorDescription>
      </Flex>

      <FormFieldTable fields={fields} />
    </Flex>
  );
};

const getShininess = (value: number) => {
  if (value === 0) {
    return "Shiny (Square in Gen 8+)";
  }

  if (value < 8) {
    return "Shiny (Star in Gen 8+)";
  }

  return "No";
};

export const IsPokemonShinyForTidSid = () => {
  const [tid, setTid] = React.useState(0);
  const [sid, setSid] = React.useState(0);
  const [pid, setPid] = React.useState(0);

  const value = shinyValue(tid, sid, pid);
  const targetTsv = tsv(tid, sid);
  const targetPsv = psv(pid);

  const fields: Field[] = [
    {
      label: "TID",
      input: (
        <NumberInput
          numType="decimal"
          value={tid}
          onChange={(value) => setTid(clamp(value, MAX_U16))}
        />
      ),
    },
    {
      label: "SID",
      input: (
        <NumberInput
          numType="decimal"
          value={sid}
          onChange={(value) => setSid(clamp(value, MAX_U16))}
        />
      ),
    },
    {
      label: "PID",
      input: (
        <NumberInput
          numType="hex"
          value={pid}
          onChange={(value) => setPid(clamp(value, MAX_U32))}
        />
      ),
    },
    {
      label: "Shininess",
      input: <OutputValue>{getShininess(value)}</OutputValue>,
    },
    {
      label: "TSV",
      input: <OutputValue>{formatId(targetTsv)}</OutputValue>,
    },
    {
      label: "PSV",
      input: <OutputValue>{formatId(targetPsv)}</OutputValue>,
    },
  ];

  return (
    <Flex vertical gap={16}>
      <Flex vertical gap={4}>
        <Typography.Title level={5} mv={0}>
          Is Pokemon Shiny for given TID/SID?
        </Typography.Title>
        <CalculatorDescription>
          Determine if target Pokemon will be shiny for given TID/SID.{" "}
          <a href="TODO link">Find your SID</a>
        </CalculatorDescription>
      </Flex>

      <FormFieldTable fields={fields} />
    </Flex>
  );
};
