use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Config {
    pub(crate) board_width: usize,
    pub(crate) tetrimino_spawn_position: (usize, usize),
    pub(crate) tetrimino_variants: HashMap<String, Tetrimino>,
    pub(crate) preview_next_count: usize,
    pub(crate) spawn_zone_border_color: String,
}

pub struct Tetrimino {
    pub color: String,
    pub orientations: Vec<Vec<(usize, usize)>>,
}

impl Config {
    pub fn get_tetrimino_color(&self, identifier: &str) -> JsValue {
        JsValue::from_str(self.tetrimino_variants[identifier].color.as_str())
    }

    pub fn get_spawn_zone_border_height(&self) -> usize {
        self.tetrimino_spawn_position.0
            + self
                .tetrimino_variants
                .values()
                .map(|tetrimino| {
                    tetrimino.orientations[0]
                        .iter()
                        .map(|(x, _)| x)
                        .min()
                        .unwrap()
                })
                .min()
                .unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut tetrimino_variants = HashMap::new();
        tetrimino_variants.insert(
            "I".to_string(),
            Tetrimino {
                color: "cyan".to_string(),
                orientations: vec![
                    vec![(1, 0), (1, 1), (1, 2), (1, 3)],
                    vec![(0, 1), (1, 1), (2, 1), (3, 1)],
                    vec![(2, 0), (2, 1), (2, 2), (2, 3)],
                    vec![(0, 2), (1, 2), (2, 2), (3, 2)],
                ],
            },
        );
        tetrimino_variants.insert(
            "O".to_string(),
            Tetrimino {
                color: "yellow".to_string(),
                orientations: vec![vec![(1, 1), (1, 2), (2, 1), (2, 2)]],
            },
        );

        Self {
            board_width: 10,
            tetrimino_spawn_position: (19, 3),
            tetrimino_variants,
            preview_next_count: 3,
            spawn_zone_border_color: "red".to_string(),
        }
    }
}
