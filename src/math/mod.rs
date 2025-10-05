mod precision;
pub mod vec2;
pub mod vec3;

pub use precision::Real;
pub use vec2::Vec2;
pub use vec3::Vec3;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Failed to create an orthonormal basis. The input vectors may be parallel.")]
    OrthonormalBasisError,
}
