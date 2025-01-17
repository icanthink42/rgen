use rgen_base::{Block, Pos};

use crate::{rng::Random, Placer, Rng, World};

pub struct Splatter {
  pub replace: Block,
  pub place:   Block,

  pub attempts: u32,
}

impl Placer for Splatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut World, rng: &mut Rng, pos: Pos) {
    for _ in 0..self.attempts {
      let pos = pos + Pos::new(rng.rand_inclusive(-8, 8), 0, rng.rand_inclusive(-8, 8));

      if world.get(pos) == self.replace {
        world.set(pos, self.place);
      }
    }
  }
}
