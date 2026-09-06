import { useState } from "react";
import {
  Button,
  InteractableMap,
  MapGlow,
  type MapFeature,
} from "~/components";
import styled from "@emotion/styled";
import { type FeebasTile, getFeebasTiles } from "./feebasMapData";

const tiles = getFeebasTiles();
const selectableTiles = tiles.filter((tile) => tile.canContainFeebas);
const TILE_WIDTH_PERCENT = 100 / 40;
const TILE_HEIGHT_PERCENT = 100 / 100;

export type FeebasMapProps = {
  selectedTiles?: number[];
  setSelectedTiles: (tiles: number[]) => void;
  canOnlySelectOne?: boolean;
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

const FeebasMapContainer = styled.div({
  width: "100%",
  maxWidth: 400,
});

export const FeebasTilesSelector = ({
  selectedTiles,
  setSelectedTiles,
  canOnlySelectOne = false,
}: FeebasMapProps) => {
  const [internalSelectedTileKeys, setSelectedTileKeys] = useState<Set<string>>(
    () => new Set(),
  );
  const selectedTileKeys =
    selectedTiles == null
      ? internalSelectedTileKeys
      : new Set(
          selectableTiles
            .filter((tile) => selectedTiles.includes(tile.cycleCounter))
            .map(getTileKey),
        );

  const toggleTile = (tile: FeebasTile) => {
    const tileKey = getTileKey(tile);
    const nextSelectedTileKeys = new Set(selectedTileKeys);

    if (nextSelectedTileKeys.has(tileKey)) {
      nextSelectedTileKeys.delete(tileKey);
    } else {
      if (canOnlySelectOne) {
        nextSelectedTileKeys.clear();
      }
      nextSelectedTileKeys.add(tileKey);
    }

    if (selectedTiles == null) {
      setSelectedTileKeys(nextSelectedTileKeys);
    }
    setSelectedTiles(
      selectableTiles
        .filter((selectableTile) =>
          nextSelectedTileKeys.has(getTileKey(selectableTile)),
        )
        .map((tile) => tile.cycleCounter),
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
    <FeebasMapContainer>
      <InteractableMap
        maxViewportHeight={400}
        alt="Route 119 Feebas fishing tiles"
        features={features}
        src="/images/Emerald/Wild/FeebasMap.png"
      />
    </FeebasMapContainer>
  );
};

const TileViewport = styled.svg({
  display: "block",
  width: "100%",
  aspectRatio: "1",
  overflow: "hidden",
  imageRendering: "pixelated",
});

export const FeebasTileVisualizer = ({
  selectedTileCycle,
}: {
  selectedTileCycle: number;
}) => {
  const tile = tiles.find((tile) => tile.cycleCounter === selectedTileCycle);
  if (tile == null) {
    return null;
  }

  const viewportSize = 20;
  const mapWidth = 100 / TILE_WIDTH_PERCENT;
  const mapHeight = 100 / TILE_HEIGHT_PERCENT;
  const x = Math.max(
    0,
    Math.min(
      tile.websiteImageX + 0.5 - viewportSize / 2,
      mapWidth - viewportSize,
    ),
  );
  const y = Math.max(
    0,
    Math.min(
      tile.websiteImageY + 0.5 - viewportSize / 2,
      mapHeight - viewportSize,
    ),
  );

  return (
    <TileViewport
      viewBox={`${x} ${y} ${viewportSize} ${viewportSize}`}
      role="img"
      aria-label={`Route 119 Feebas map around tile cycle ${selectedTileCycle}`}
    >
      <image
        href="/images/Emerald/Wild/FeebasMap.png"
        width={mapWidth}
        height={mapHeight}
        preserveAspectRatio="none"
      />
    </TileViewport>
  );
};

export const FeebasTileVisualizerButton = ({
  selectedTileCycle,
}: {
  selectedTileCycle: number;
}) => {
  const [isVisible, setIsVisible] = useState(false);
  const tile = tiles.find((tile) => tile.cycleCounter === selectedTileCycle);
  if (tile == null) {
    return null;
  }

  return (
    <>
      <Button
        trackerId="feebas-tile-visualizer-toggle"
        aria-expanded={isVisible}
        onClick={() => setIsVisible((visible) => !visible)}
      >
        {`Tile ${tile.ingameX},${tile.ingameY}`}
      </Button>
      {isVisible && (
        <FeebasTileVisualizer selectedTileCycle={selectedTileCycle} />
      )}
    </>
  );
};
