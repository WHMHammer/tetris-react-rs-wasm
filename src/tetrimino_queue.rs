use rand::{rng, seq::SliceRandom};
use std::{collections::VecDeque, ops::Deref};

use crate::config::Config;

pub struct TetriminoQueue {
    queue: VecDeque<String>,
}

impl TetriminoQueue {
    pub fn new(config: &Config) -> Self {
        let mut queue = Self {
            queue: VecDeque::with_capacity(std::cmp::max(
                config.tetrimino_variants.len(),
                config.preview_next_count + 1,
            )),
        };
        while queue.queue.capacity() - queue.queue.len() >= config.tetrimino_variants.len() {
            queue.populate(&config);
        }
        queue
    }

    fn populate(&mut self, config: &Config) {
        let mut rng = rng();
        let mut tetriminoes = config
            .tetrimino_variants
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        tetriminoes.shuffle(&mut rng);
        self.queue.extend(tetriminoes.into_iter());
    }
}

impl Deref for TetriminoQueue {
    type Target = VecDeque<String>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}
