use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;

use tetris_rs::{Config, Game, Position};

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameWrapper {
    id: u128,
    game: Arc<RwLock<Game>>,
}

#[wasm_bindgen]
impl GameWrapper {
    fn new(config: Config) -> Self {
        Self {
            game: Arc::new(RwLock::new(Game::new(config))),
            id: rand::random::<u128>(),
        }
    }

    pub fn default() -> Self {
        Self::new(Config::default())
    }

    pub fn id(&self) -> String {
        format!("{:X}", self.id)
    }

    pub fn danger_zone_border(&self) -> usize {
        match self.game.read() {
            Ok(game) => game.danger_zone_border(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn board_width(&self) -> usize {
        match self.game.read() {
            Ok(game) => game.board()[0].len(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn board_height(&self) -> usize {
        match self.game.read() {
            Ok(game) => game.board().len(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn board_buffer(&self) -> Vec<JsValue> {
        match self.game.read() {
            Ok(game) => game
                .board()
                .iter()
                .flatten()
                .map(|id| match id.as_ref() {
                    Some(id) => JsValue::from_str(id.as_str()),
                    None => JsValue::null(),
                })
                .collect(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn current_tetrimino_id(&self) -> String {
        match self.game.read() {
            Ok(game) => game.current_tetrimino_id().clone(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn current_tetrimino_cells_board_buffer_indices(&self) -> Vec<usize> {
        match self.game.read() {
            Ok(game) => game
                .current_tetrimino_cells_iter()
                .map(|Position { x, y }| x * game.board()[0].len() + y)
                .collect(),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn is_over(&self) -> bool {
        match self.game.read() {
            Ok(game) => game.is_over(),
            Err(err) => panic!("{}", err),
        }
    }

    fn get_closure(&self, f: impl Fn(&mut Game) + 'static) -> JsValue {
        let Self { game, id } = self.clone();
        let closure: Closure<dyn Fn() -> Self> = Closure::new(move || {
            match game.write() {
                Ok(mut game) => f(&mut game),
                Err(err) => panic!("{}", err),
            }
            Self {
                game: game.clone(),
                id: id.clone(),
            }
        });
        let js_closure = closure.as_ref().clone();
        closure.forget();
        js_closure
    }

    pub fn get_rotate_forwards_closure(&self) -> JsValue {
        self.get_closure(|game| game.rotate_forwards())
    }

    pub fn get_rotate_backwards_closure(&self) -> JsValue {
        self.get_closure(|game| game.rotate_backwards())
    }

    pub fn get_move_down_closure(&self) -> JsValue {
        self.get_closure(|game| {
            game.move_down();
        })
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
}
