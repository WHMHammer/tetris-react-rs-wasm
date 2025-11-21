use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;

use crate::{board::Board, config::Config, tetrimino_queue::TetriminoQueue};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game(Arc<RwLock<_Game>>);

#[wasm_bindgen]
impl Game {
    pub fn new(config: Option<Config>) -> Self {
        let config = config.unwrap_or_default();
        let board = Board::new(&config);
        let tetrimino_queue = TetriminoQueue::new(&config);
        let current_tetrimino_position = config.tetrimino_spawn_position.clone();
        Self(Arc::new(RwLock::new(_Game {
            config,
            board,
            tetrimino_queue,
            current_tetrimino_position,
            current_tetrimino_orientation: 0,
        })))
    }

    pub fn get_board_display_width(&self) -> usize {
        match self.0.read() {
            Ok(game) => game.board.width(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_board_display_height(&self) -> usize {
        match self.0.read() {
            Ok(game) => game.board.height(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_spawn_zone_border_height(&self) -> usize {
        match self.0.read() {
            Ok(game) => game.config.get_spawn_zone_border_height(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_spawn_zone_border_color(&self) -> JsValue {
        match self.0.read() {
            Ok(game) => JsValue::from_str(game.config.spawn_zone_border_color.as_str()),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_display_buffer(&self) -> Vec<JsValue> {
        match self.0.read() {
            Ok(game) => {
                let mut buffer = Vec::with_capacity(game.board.width() * game.board.height());
                buffer.extend(game.board.iter().flatten().map(|id| {
                    id.as_ref().map_or(JsValue::null(), |id| {
                        JsValue::from_str(game.config.tetrimino_variants[id].color.as_str())
                    })
                }));
                for &(x, y) in game.config.tetrimino_variants[game.current_tetrimino()].orientations
                    [game.current_tetrimino_orientation]
                    .iter()
                {
                    buffer[(x + game.current_tetrimino_position.0) * game.board.width()
                        + (y + game.current_tetrimino_position.1)] =
                        game.config.get_tetrimino_color(game.current_tetrimino());
                }
                buffer
            }
            Err(err) => panic!("{}", err),
        }
    }

    pub fn rotate(&self) -> Self {
        match self.0.write() {
            Ok(mut game) => {
                game.current_tetrimino_orientation += 1;
                if game.current_tetrimino_orientation
                    == game
                        .config
                        .get_tetrimino_orientations(&game.current_tetrimino())
                        .len()
                {
                    game.current_tetrimino_orientation = 0;
                }
            }
            Err(err) => panic!("{}", err),
        }
        self.clone()
    }
}

struct _Game {
    config: Config,
    board: Board,
    tetrimino_queue: TetriminoQueue,
    current_tetrimino_position: (usize, usize),
    current_tetrimino_orientation: usize,
}

impl _Game {
    fn current_tetrimino(&self) -> &String {
        self.tetrimino_queue.front().unwrap()
    }
}
