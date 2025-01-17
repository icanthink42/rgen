use rgen_base::{Block, Pos};

use crate::{Placer, Random, Rng, World};

pub struct BasicTree {
  pub trunk:  Block,
  pub leaves: Block,
}

impl Placer for BasicTree {
  fn radius(&self) -> u8 { 2 }

  fn amount_per_chunk(&self) -> u32 { 16 }

  fn place(&self, world: &mut World, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(4, 7);
    let min_y = rng.rand_inclusive(-2, -1);

    if pos.y as i32 + height as i32 + 2 >= 255 || pos.y <= 1 {
      return;
    }

    for y in min_y..=1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if x.abs() == 2 && z.abs() == 2 {
            continue;
          }
          // Make the top layer smaller.
          if y == 1 && (x.abs() == 2 || z.abs() == 2) {
            continue;
          }

          world.set(pos + Pos::new(x, (y + height) as u8, z), self.leaves);
        }
      }
    }

    for y in 0..height as u8 {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
  }
}
