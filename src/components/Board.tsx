import { FC, useMemo } from "react";
import { CellDisplay, GameWrapper } from "../../pkg/tetris_rs_wasm";
import { CellAttribute } from "../../pkg/tetris_rs_wasm_bg";

interface BoardProps {
  game: GameWrapper;
}

export const BoardComponent: FC<BoardProps> = ({ game }) => {
  const height = game.board_height();
  const width = game.board_width();
  const boardReverseXIndices = useMemo(
    () => [...Array(height).keys()].reverse(),
    [height],
  );
  const boardYIndices = useMemo(() => [...Array(width).keys()], [width]);
  const boardDisplay: (CellDisplay | undefined)[][] = game.get_board_display();
  const dangerZoneBorder = game.danger_zone_border();
  const tetriminoVariantsNames = game.tetrimino_variants_names();

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
              if (boardDisplay[x][y] !== undefined) {
                classNames.push(
                  "board__cell--" +
                    tetriminoVariantsNames[boardDisplay[x][y].tetrimino_id],
                );
                switch (boardDisplay[x][y].attribute) {
                  case CellAttribute.IsCurrent:
                    classNames.push("board__cell--current");
                    break;

                  case CellAttribute.IsGhost:
                    classNames.push("board__cell--ghost");
                    break;

                  default:
                    break;
                }
              }
              return <div key={y} className={classNames.join(" ")} />;
            })}
          </div>
        );
      })}
    </div>
  );
};
