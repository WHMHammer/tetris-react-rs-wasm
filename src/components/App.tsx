import { FC, useEffect, useState } from "react";
import { BoardComponent } from "./Board";
import { GameWrapper } from "../../pkg";

export const App: FC = () => {
  const [game, setGame] = useState(GameWrapper.default());
  const gameId = game.id();

  useEffect(() => {
    const rotate: () => GameWrapper = game.get_rotate_closure();

    const windowKeydownHandler = (event: KeyboardEvent) => {
      if (event.key !== "F12") {
        event.preventDefault();
      }
      console.log(event);
      switch (event.key) {
        case "w":
          setGame(rotate());
          return;
      }
    };
    window.addEventListener("keydown", windowKeydownHandler);
    return () => {
      window.removeEventListener("keydown", windowKeydownHandler);
    };
  }, [gameId]);

  return (
    <>
      <BoardComponent
        width={game.board_width()}
        height={game.board_height()}
        buffer={game.board_buffer()}
        dangerZoneXLow={game.danger_zone_x_low()}
        currentTetriminoId={game.current_tetrimino_id()}
        currentTetriminoCellsBoardBufferIndices={game.current_tetrimino_cells_board_buffer_indices()}
      />
      <button onClick={() => setGame(GameWrapper.default())}>New Game</button>
    </>
  );
};
