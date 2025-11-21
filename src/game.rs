use wasm_bindgen::prelude::*;

use crate::{board::Board, config::Config, tetrimino_queue::TetriminoQueue};

#[wasm_bindgen]
pub struct Game {
    config: Config,
    board: Board,
    tetrimino_queue: TetriminoQueue,
    current_tetrimino_position: (usize, usize),
    current_tetrimino_orientation: usize,
}

#[wasm_bindgen]
impl Game {
    pub fn new(config: Option<Config>) -> Self {
        let config = config.unwrap_or_default();
        let board = Board::new(&config);
        let tetrimino_queue = TetriminoQueue::new(&config);
        let current_tetrimino_position = config.tetrimino_spawn_position.clone();
        Self {
            config,
            board,
            tetrimino_queue,
            current_tetrimino_position,
            current_tetrimino_orientation: 0,
        }
    }

    pub fn get_board_display_width(&self) -> usize {
        self.board.width()
    }

    pub fn get_board_display_height(&self) -> usize {
        self.board.height()
    }

    pub fn get_spawn_zone_border_height(&self) -> usize {
        self.config.get_spawn_zone_border_height()
    }

    pub fn get_spawn_zone_border_color(&self) -> JsValue {
        JsValue::from_str(self.config.spawn_zone_border_color.as_str())
    }

    pub fn get_display_buffer(&self) -> Vec<JsValue> {
        let mut buffer = Vec::with_capacity(self.board.width() * self.board.height());
        buffer.extend(self.board.iter().flatten().map(|id| {
            id.as_ref().map_or(JsValue::null(), |id| {
                JsValue::from_str(self.config.tetrimino_variants[id].color.as_str())
            })
        }));
        for &(x, y) in self.config.tetrimino_variants[self.current_tetrimino()].orientations
            [self.current_tetrimino_orientation]
            .iter()
        {
            buffer[(x + self.current_tetrimino_position.0) * self.board.width()
                + (y + self.current_tetrimino_position.1)] =
                self.config.get_tetrimino_color(self.current_tetrimino());
        }
        buffer
    }

    fn current_tetrimino(&self) -> &String {
        self.tetrimino_queue.front().unwrap()
    }
}
