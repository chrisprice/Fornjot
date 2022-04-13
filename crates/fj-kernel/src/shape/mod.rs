//! The API used for creating and manipulating shapes
//!
//! See [`Shape`], which is the main entry point to this API.

mod api;
mod geometry;
mod object;
mod stores;
mod topology;
mod validate;

pub use self::{
    api::Shape,
    geometry::Geometry,
    object::Object,
    stores::{Handle, Iter},
    topology::Topology,
    validate::{StructuralIssues, ValidationError, ValidationResult},
};