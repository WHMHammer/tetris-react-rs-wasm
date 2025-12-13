use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;

use tetris_rs::{Config, Game, Position, Shuffle};

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameWrapper {
    id: u128,
    game: Arc<RwLock<Game<Shuffler>>>,
}

#[wasm_bindgen]
impl GameWrapper {
    fn new(config: Config) -> Self {
        Self {
            game: Arc::new(RwLock::new(Game::new(config, Shuffler::new()))),
            id: rand::random::<u128>(),
        }
    }

    pub fn default() -> Self {
        Self::new(Config::default())
    }

    pub fn id(&self) -> String {
        format!("{:X}", self.id)
    }

    // config

    pub fn board_width(&self) -> usize {
        self.get(|game| game.config().board_width())
    }

    pub fn board_height(&self) -> usize {
        self.get(|game| game.config().board_height())
    }

    pub fn danger_zone_border(&self) -> usize {
        self.get(|game| game.config().danger_zone_border())
    }

    pub fn preview_next_count(&self) -> usize {
        self.get(|game| game.config().preview_next_count())
    }

    pub fn tetrimino_variants_names(&self) -> Vec<String> {
        self.get(|game| {
            let iter = game
                .config()
                .tetrimino_variants()
                .iter()
                .map(|variant| variant.name().clone());
            let mut names = Vec::with_capacity(iter.len());
            names.extend(iter);
            names
        })
    }

    // board

    pub fn get_board_display(&self) -> Vec<js_sys::Array> {
        self.get(|game| {
            let config = game.config();
            let width = config.board_width();
            let height = config.board_height();

            let board = game.board();
            let mut board_display = Vec::with_capacity(height);
            for x in 0..height {
                let board_display_row = js_sys::Array::new_with_length(width as u32);
                for y in 0..width {
                    if let Some(tetrimino_id) = board[x][y] {
                        board_display_row.set(
                            y as u32,
                            CellDisplay {
                                tetrimino_id,
                                attribute: None,
                            }
                            .into(),
                        );
                    }
                }
                // board_display.set(x as u32, board_display_row.into());
                board_display.push(board_display_row);
            }

            for Position { x, y } in
                game.tetrimino_cell_positions_iter(game.ghost_tetrimino_position())
            {
                board_display[x].set(
                    y as u32,
                    CellDisplay {
                        tetrimino_id: game.current_tetrimino_id(),
                        attribute: Some(CellAttribute::IsGhost),
                    }
                    .into(),
                );
            }

            for Position { x, y } in
                game.tetrimino_cell_positions_iter(game.current_tetrimino_position())
            {
                board_display[x].set(
                    y as u32,
                    CellDisplay {
                        tetrimino_id: game.current_tetrimino_id(),
                        attribute: Some(CellAttribute::IsCurrent),
                    }
                    .into(),
                );
            }

            board_display
        })
    }

    // tetrimino queue

    pub fn next_tetrimino_ids(&self) -> Vec<usize> {
        self.get(|game| {
            let mut n = Vec::with_capacity(game.config().preview_next_count());
            n.extend(game.next_tetrimino_ids_iter().cloned());
            n
        })
    }

    // hold

    pub fn held(&self) -> Option<usize> {
        self.get(|game| game.held())
    }

    // score

    pub fn score(&self) -> usize {
        self.get(|game| game.score())
    }

    // is over

    pub fn is_over(&self) -> bool {
        self.get(|game| game.is_over())
    }

    // operations

    pub fn get_rotate_forwards_closure(&self) -> JsValue {
        self.get_closure(|game| game.rotate_forwards())
    }

    pub fn get_rotate_backwards_closure(&self) -> JsValue {
        self.get_closure(|game| game.rotate_backwards())
    }

    pub fn get_move_down_closure(&self) -> JsValue {
        self.get_closure(|game| game.move_down())
    }

    pub fn get_drop_closure(&self) -> JsValue {
        self.get_closure(|game| game.drop())
    }

    pub fn get_move_left_closure(&self) -> JsValue {
        self.get_closure(|game| game.move_left())
    }

    pub fn get_move_right_closure(&self) -> JsValue {
        self.get_closure(|game| game.move_right())
    }

    pub fn get_hold_closure(&mut self) -> JsValue {
        self.get_closure(|game| game.hold())
    }

    // helper functions

    fn get<T>(&self, f: impl Fn(&Game<Shuffler>) -> T) -> T {
        match self.game.read() {
            Ok(game) => f(&game),
            Err(err) => panic!("{}", err),
        }
    }

    fn get_closure<T>(&self, f: impl Fn(&mut Game<Shuffler>) -> T + 'static) -> JsValue {
        let Self { game, id } = self.clone();
        let closure: Closure<dyn Fn() -> Self> = Closure::new(move || {
            match game.write() {
                Ok(mut game) => {
                    f(&mut game);
                }
                Err(err) => panic!("{}", err),
            }
            Self {
                game: game.clone(),
                id,
            }
        });
        let js_closure = closure.as_ref().clone();
        closure.forget();
        js_closure
    }
}

struct Shuffler(rand::prelude::ThreadRng);

impl Shuffler {
    fn new() -> Self {
        Self(rand::rng())
    }
}

impl Shuffle for Shuffler {
    fn shuffle(&mut self, v: &mut Vec<usize>) {
        use rand::seq::SliceRandom;
        v.shuffle(&mut self.0);
    }
}

#[wasm_bindgen]
pub struct CellDisplay {
    pub tetrimino_id: usize,
    pub attribute: Option<CellAttribute>,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum CellAttribute {
    IsCurrent = 1,
    IsGhost,
}
