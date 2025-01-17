mod octaved;
mod perlin;

pub use octaved::OctavedNoise;
pub use perlin::PerlinNoise;

pub trait NoiseGenerator {
  fn generate(&self, x: f64, y: f64, seed: u64) -> f64;
}
