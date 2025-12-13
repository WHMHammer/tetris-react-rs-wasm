import { FC, useEffect, useMemo, useState } from "react";
import { BoardComponent } from "./Board";
import { GameWrapper } from "../../pkg";

export const App: FC = () => {
  const [game, setGame] = useState(GameWrapper.default());
  const gameId = game.id();
  const tetriminoVariantsNames = useMemo(
    () => game.tetrimino_variants_names(),
    [gameId],
  );
  const held = game.held();

  useEffect(() => {
    const rotateForwards: () => GameWrapper =
      game.get_rotate_forwards_closure();
    const rotateBackwards: () => GameWrapper =
      game.get_rotate_backwards_closure();
    const moveDown: () => GameWrapper = game.get_move_down_closure();
    const drop: () => GameWrapper = game.get_drop_closure();
    const moveLeft: () => GameWrapper = game.get_move_left_closure();
    const moveRight: () => GameWrapper = game.get_move_right_closure();
    const hold: () => GameWrapper = game.get_hold_closure();

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

        case "h":
          setGame(hold());
          return;
      }
    };
    window.addEventListener("keydown", windowKeydownHandler);
    return () => window.removeEventListener("keydown", windowKeydownHandler);
  }, [gameId]);

  return (
    <>
      <BoardComponent game={game} />
      <button onClick={() => setGame(GameWrapper.default())}>New Game</button>
      <div>
        Score: {game.score()}
        {game.is_over() && " Game Over!"}
      </div>
      <div>Held: {held !== undefined && tetriminoVariantsNames[held]}</div>
      <div>
        Next:{" "}
        {Array.from(game.next_tetrimino_ids())
          .map((tetrimino_id) => tetriminoVariantsNames[tetrimino_id])
          .join(" ")}
      </div>
    </>
  );
};
