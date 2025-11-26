import { FC, useMemo } from "react";

interface BoardProps {
  width: number;
  height: number;
  buffer: (string | null)[];
  dangerZoneBorder: number;
  currentTetriminoId: string;
  currentTetriminoCellsBoardBufferIndices: Uint32Array;
  ghostTetriminoCellsBoardBufferIndices: Uint32Array;
}

export const BoardComponent: FC<BoardProps> = ({
  width,
  height,
  buffer,
  dangerZoneBorder,
  currentTetriminoId,
  currentTetriminoCellsBoardBufferIndices,
  ghostTetriminoCellsBoardBufferIndices,
}) => {
  const boardReverseXIndices = useMemo(
    () => [...Array(height).keys()].reverse(),
    [height],
  );
  const boardYIndices = useMemo(() => [...Array(width).keys()], [width]);
  ghostTetriminoCellsBoardBufferIndices.forEach((i) => {
    buffer[i] = "ghost";
  });
  currentTetriminoCellsBoardBufferIndices.forEach((i) => {
    buffer[i] = currentTetriminoId;
  });

  return (
    <div className="board">
      {boardReverseXIndices.map((x) => {
        const classNames = ["board__row"];
        if (x >= dangerZoneBorder) {
          classNames.push("board__row--danger");
        }
        return (
          <div key={x} className={classNames.join(" ")}>
            {boardYIndices.map((y) => {
              const classNames = ["board__cell"];
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
