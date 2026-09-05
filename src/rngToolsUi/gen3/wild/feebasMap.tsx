import { useState } from "react";
import { InteractableMap, MapGlow, type MapFeature } from "~/components";
import styled from "@emotion/styled";
import { type FeebasTile, getFeebasTiles } from "./feebasMapData";

const tiles = getFeebasTiles();
const selectableTiles = tiles.filter((tile) => tile.canContainFeebas);
const TILE_WIDTH_PERCENT = 100 / 40;
const TILE_HEIGHT_PERCENT = 100 / 100;

export type FeebasMapProps = {
  setSelectedTiles: (tiles: FeebasTile[]) => void;
};

const getTileKey = (tile: FeebasTile) =>
  `${tile.websiteImageX},${tile.websiteImageY}`;

const TileGlow = styled(MapGlow)<{
  $instability: FeebasTile["instability"];
  $selected: boolean;
}>(({ $instability, $selected }) => ({
  backgroundColor: $instability === 0 ? "lightgreen" : "red",
  opacity: $selected ? 1 : $instability === 0 ? 0 : 0.2,
}));

export const FeebasMap = ({ setSelectedTiles }: FeebasMapProps) => {
  const [selectedTileKeys, setSelectedTileKeys] = useState<Set<string>>(
    () => new Set(),
  );

  const toggleTile = (tile: FeebasTile) => {
    const tileKey = getTileKey(tile);
    const nextSelectedTileKeys = new Set(selectedTileKeys);

    if (nextSelectedTileKeys.has(tileKey)) {
      nextSelectedTileKeys.delete(tileKey);
    } else {
      nextSelectedTileKeys.add(tileKey);
    }

    setSelectedTileKeys(nextSelectedTileKeys);
    setSelectedTiles(
      selectableTiles.filter((selectableTile) =>
        nextSelectedTileKeys.has(getTileKey(selectableTile)),
      ),
    );
  };

  const features = selectableTiles.map((tile): MapFeature => {
    const x = tile.websiteImageX * TILE_WIDTH_PERCENT;
    const y = tile.websiteImageY * TILE_HEIGHT_PERCENT;
    const isSelected = selectedTileKeys.has(getTileKey(tile));

    return {
      type: "polygon",
      points: [
        { x, y },
        { x: x + TILE_WIDTH_PERCENT, y },
        { x: x + TILE_WIDTH_PERCENT, y: y + TILE_HEIGHT_PERCENT },
        { x, y: y + TILE_HEIGHT_PERCENT },
      ],
      node: (
        <TileGlow
          $instability={tile.instability}
          $selected={isSelected}
          onClick={() => toggleTile(tile)}
          role="button"
        />
      ),
    };
  });

  return (
    <InteractableMap
      alt="Route 119 Feebas fishing tiles"
      features={features}
      src="/images/Emerald/Wild/FeebasMap.png"
    />
  );
};
