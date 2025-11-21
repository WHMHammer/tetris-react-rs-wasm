import { FC, useEffect, useState } from "react";
import { BoardComponent } from "./Board";
import { Game } from "../../pkg";

export const App: FC = () => {
  const [game, setGame] = useState(Game.new());
  useEffect(() => {
    const windowKeydownHandler = (event: KeyboardEvent) => {
      if (event.key !== "F12") {
        event.preventDefault();
      }
      console.log(event);
      switch (event.key) {
        case "w":
          setGame(game.rotate());
          return;
      }
    };
    window.addEventListener("keydown", windowKeydownHandler);
    return () => {
      window.removeEventListener("keydown", windowKeydownHandler);
    };
  }, [game]);

  return (
    <>
      <BoardComponent
        displayWidth={game.get_board_display_width()}
        displayHeight={game.get_board_display_height()}
        spawnZoneBorderHeight={game.get_spawn_zone_border_height()}
        spawnZoneBorderColor={game.get_spawn_zone_border_color()}
        buffer={game.get_display_buffer()}
      />
      <button onClick={() => setGame(Game.new())}>New Game</button>
    </>
  );
};
