import { FC, useState } from "react";
import { BoardComponent } from "./Board";
import { Game } from "../../pkg";

export const App: FC = () => {
  const [game] = useState(Game.new());

  return (
    <>
      <BoardComponent
        displayWidth={game.get_board_display_width()}
        displayHeight={game.get_board_display_height()}
        spawnZoneBorderHeight={game.get_spawn_zone_border_height()}
        spawnZoneBorderColor={game.get_spawn_zone_border_color()}
        buffer={game.get_display_buffer()}
      />
    </>
  );
};
