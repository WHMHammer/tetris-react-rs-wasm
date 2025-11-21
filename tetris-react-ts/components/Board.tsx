import { CSSProperties, FC, useMemo } from "react";

interface BoardProps {
  displayWidth: number;
  displayHeight: number;
  spawnZoneBorderHeight: number;
  spawnZoneBorderColor: string;
  buffer: string[];
}

export const BoardComponent: FC<BoardProps> = ({
  displayWidth,
  displayHeight,
  spawnZoneBorderHeight,
  spawnZoneBorderColor,
  buffer,
}) => {
  const boardReverseXIndices = useMemo(
    () => [...Array(displayHeight).keys()].reverse(),
    [displayHeight],
  );
  const boardYIndices = useMemo(
    () => [...Array(displayWidth).keys()],
    [displayWidth],
  );
  console.log(buffer);

  return (
    <div className="board">
      {boardReverseXIndices.map((x) => (
        <div key={x} className="board__row">
          {boardYIndices.map((y) => {
            const classNames = ["board__cell"];
            if (x >= spawnZoneBorderHeight) {
              classNames.push("board__cell--spawn-zone");
            }
            const style: CSSProperties = {
              backgroundColor: buffer[x * displayWidth + y],
            };
            if (x === spawnZoneBorderHeight) {
              style.borderBottomColor = spawnZoneBorderColor;
            } else if (x === spawnZoneBorderHeight - 1) {
              style.borderTopColor = spawnZoneBorderColor;
            }
            return (
              <div key={y} className={classNames.join(" ")} style={style} />
            );
          })}
        </div>
      ))}
    </div>
  );
};
