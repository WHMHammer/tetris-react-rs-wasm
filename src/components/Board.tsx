import { FC, useMemo } from "react";

interface BoardProps {
  width: number;
  height: number;
  buffer: (string | null)[];
  dangerZoneXLow: number;
  currentTetriminoId: string;
  currentTetriminoCellsBoardBufferIndices: Uint32Array;
}

export const BoardComponent: FC<BoardProps> = ({
  width,
  height,
  buffer,
  dangerZoneXLow,
  currentTetriminoId,
  currentTetriminoCellsBoardBufferIndices,
}) => {
  const boardReverseXIndices = useMemo(
    () => [...Array(height).keys()].reverse(),
    [height],
  );
  const boardYIndices = useMemo(() => [...Array(width).keys()], [width]);
  currentTetriminoCellsBoardBufferIndices.forEach((i) => {
    buffer[i] = currentTetriminoId;
  });

  return (
    <div className="board">
      {boardReverseXIndices.map((x) => {
        const classNames = ["board__row"];
        if (x === dangerZoneXLow) {
          classNames.push("board__row--danger-zone-bottom");
        } else if (x + 1 === dangerZoneXLow) {
          classNames.push("board__row--safe-zone-top");
        }
        const isInDangerZone = x >= dangerZoneXLow;
        return (
          <div key={x} className={classNames.join(" ")}>
            {boardYIndices.map((y) => {
              const classNames = ["board__cell"];
              if (isInDangerZone) {
                classNames.push("board__cell--danger");
              }
              const id = buffer[x * width + y];
              if (id) {
                classNames.push("board__cell--" + id);
              }
              return <div key={y} className={classNames.join(" ")} />;
            })}
          </div>
        );
      })}
    </div>
  );
};
