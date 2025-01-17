use rgen_base::{Block, Chunk, ChunkPos, Pos};

/// A World is a proxy around a specific chunk, that lets you "place" a block in
/// the world, but will end up only setting blocks in that one chunk.
pub struct World<'a> {
  chunk_pos: ChunkPos,
  chunk:     &'a mut Chunk,
}

impl<'a> World<'a> {
  pub fn new(chunk_pos: ChunkPos, chunk: &'a mut Chunk) -> World<'a> { World { chunk_pos, chunk } }

  pub fn get(&self, pos: Pos) -> Block {
    if pos.in_chunk(self.chunk_pos) {
      self.chunk.get(pos.chunk_rel())
    } else {
      Block::AIR
    }
  }

  pub fn set(&mut self, pos: Pos, block: Block) {
    if pos.in_chunk(self.chunk_pos) {
      self.chunk.set(pos.chunk_rel(), block);
    }
  }

  pub fn top_block(&self, pos: Pos) -> Pos { self.top_block_excluding(pos, &[]) }

  /// Returns the highest block that is not air and not in the `exclude` list.
  pub fn top_block_excluding(&self, pos: Pos, exclude: &[Block]) -> Pos {
    let mut y = 255;
    while y > 0 {
      let block = self.get(pos.with_y(y));
      if block != Block::AIR && !exclude.contains(&block) {
        break;
      }
      y -= 1;
    }
    pos.with_y(y)
  }
}
