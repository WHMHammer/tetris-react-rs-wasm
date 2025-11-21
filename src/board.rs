use crate::config::Config;
use std::{collections::VecDeque, ops::Deref};

pub struct Board {
    board: VecDeque<Vec<Option<String>>>,
}

impl Board {
    pub fn new(config: &Config) -> Self {
        let mut board = VecDeque::with_capacity(
            config.tetrimino_spawn_position.0
                + config
                    .tetrimino_variants
                    .values()
                    .map(|tetrimino| {
                        tetrimino
                            .orientations
                            .iter()
                            .map(|positions| positions.iter().map(|(x, _)| x).max().unwrap())
                            .max()
                            .unwrap()
                    })
                    .max()
                    .unwrap()
                + 1,
        );
        board.resize_with(board.capacity(), || {
            let mut row = Vec::with_capacity(config.board_width);
            row.resize(config.board_width, None);
            row
        });
        Self { board }
    }

    pub fn width(&self) -> usize {
        self.board.front().unwrap().len()
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }
}

impl Deref for Board {
    type Target = VecDeque<Vec<Option<String>>>;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}
