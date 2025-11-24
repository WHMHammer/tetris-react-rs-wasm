import { FC, useEffect, useState } from "react";
import { BoardComponent } from "./Board";
import { GameWrapper } from "../../pkg";

export const App: FC = () => {
  const [game, setGame] = useState(GameWrapper.default());
  const gameId = game.id();

  useEffect(() => {
    const rotateForwards: () => GameWrapper =
      game.get_rotate_forwards_closure();
    const rotateBackwards: () => GameWrapper =
      game.get_rotate_backwards_closure();
    const moveDown: () => GameWrapper = game.get_move_down_closure();
    const drop: () => GameWrapper = game.get_drop_closure();
    const moveLeft: () => GameWrapper = game.get_move_left_closure();
    const moveRight: () => GameWrapper = game.get_move_right_closure();

    const windowKeydownHandler = (event: KeyboardEvent) => {
      if (event.key !== "F12") {
        event.preventDefault();
      }

      switch (event.key) {
        case "e":
          setGame(rotateForwards());
          return;

        case "q":
          setGame(rotateBackwards());
          return;

        case "s":
          setGame(moveDown());
          return;

        case " ":
          setGame(drop());
          return;

        case "a":
          setGame(moveLeft());
          return;

        case "d":
          setGame(moveRight());
          return;
      }
    };
    window.addEventListener("keydown", windowKeydownHandler);
    return () => window.removeEventListener("keydown", windowKeydownHandler);
  }, [gameId]);

  return (
    <>
      <BoardComponent
        width={game.board_width()}
        height={game.board_height()}
        buffer={game.board_buffer()}
        dangerZoneBorder={game.danger_zone_border()}
        currentTetriminoId={game.current_tetrimino_id()}
        currentTetriminoCellsBoardBufferIndices={game.current_tetrimino_cells_board_buffer_indices()}
      />
      <div>
        <button onClick={() => setGame(GameWrapper.default())}>New Game</button>
        {game.is_over() && "Game Over!"}
      </div>
    </>
  );
};
