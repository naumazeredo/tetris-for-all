// @TODO abstract PCG
use crate::rand_core::RngCore;

use crate::app::ImDraw;
use crate::game::pieces::{
    PieceType,
    PIECES
};
use super::RandomizerTrait;

#[derive(Clone, Debug, ImDraw)]
pub struct RandomizerFullRandom {
    rng: rand_pcg::Pcg32,
}

impl RandomizerFullRandom {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(seed, 0xa02bdbf7bb3c0a7),
        }
    }
}

impl RandomizerTrait for RandomizerFullRandom {
    fn reset(&mut self) {}

    fn next_piece(&mut self) -> PieceType {
        let piece_id = (self.rng.next_u32() % 7) as usize;
        PIECES[piece_id]
    }
}
