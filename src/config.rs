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
    pub fn get_tetrimino_color(&self, id: &String) -> JsValue {
        JsValue::from_str(self.tetrimino_variants[id].color.as_str())
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

    pub fn get_tetrimino_orientations(&self, id: &String) -> &Vec<Vec<(usize, usize)>> {
        &self.tetrimino_variants[id].orientations
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
            "J".to_string(),
            Tetrimino {
                color: "blue".to_string(),
                orientations: vec![
                    vec![(1, 0), (1, 1), (1, 2), (2, 0)],
                    vec![(0, 1), (1, 1), (2, 1), (2, 2)],
                    vec![(0, 2), (1, 0), (1, 1), (1, 2)],
                    vec![(0, 0), (0, 1), (1, 1), (2, 1)],
                ],
            },
        );
        tetrimino_variants.insert(
            "L".to_string(),
            Tetrimino {
                color: "orange".to_string(),
                orientations: vec![
                    vec![(1, 0), (1, 1), (1, 2), (2, 2)],
                    vec![(0, 1), (0, 2), (1, 1), (2, 1)],
                    vec![(0, 0), (1, 0), (1, 1), (1, 2)],
                    vec![(0, 1), (1, 1), (2, 0), (2, 1)],
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
        tetrimino_variants.insert(
            "S".to_string(),
            Tetrimino {
                color: "green".to_string(),
                orientations: vec![
                    vec![(1, 0), (1, 1), (2, 1), (2, 2)],
                    vec![(0, 2), (1, 1), (1, 2), (2, 1)],
                    vec![(0, 0), (0, 1), (1, 1), (1, 2)],
                    vec![(0, 1), (1, 0), (1, 1), (2, 0)],
                ],
            },
        );
        tetrimino_variants.insert(
            "T".to_string(),
            Tetrimino {
                color: "purple".to_string(),
                orientations: vec![
                    vec![(1, 0), (1, 1), (1, 2), (2, 1)],
                    vec![(0, 1), (1, 1), (1, 2), (2, 1)],
                    vec![(0, 1), (1, 0), (1, 1), (1, 2)],
                    vec![(0, 1), (1, 0), (1, 1), (2, 1)],
                ],
            },
        );
        tetrimino_variants.insert(
            "Z".to_string(),
            Tetrimino {
                color: "red".to_string(),
                orientations: vec![
                    vec![(1, 1), (1, 2), (2, 0), (2, 1)],
                    vec![(0, 1), (1, 1), (1, 2), (2, 2)],
                    vec![(0, 1), (0, 2), (1, 0), (1, 1)],
                    vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                ],
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
