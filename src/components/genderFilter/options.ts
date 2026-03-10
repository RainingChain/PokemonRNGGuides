import { GenderRatio } from "~/rngTools";
import { gender, getPossibleGenders } from "~/types/gender";

export const genderOptions = ([null, ...gender] as const).map((gen) => ({
  label: gen ?? ("Any" as const),
  value: gen,
}));

export const getGenderFilterOptions = (genderRatio?: GenderRatio) => {
  if (genderRatio == null) {
    return genderOptions;
  }

  const possibleGenders = getPossibleGenders(genderRatio);
  const permitNull = possibleGenders.length > 1;
  return genderOptions.filter((option) => {
    if (option.value == null) {
      return permitNull;
    }
    return possibleGenders.includes(option.value);
  });
};
