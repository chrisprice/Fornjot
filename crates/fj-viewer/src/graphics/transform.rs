use bytemuck::{Pod, Zeroable};

use crate::camera::Camera;
use fj_math::Transform as fjTransform;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Transform(pub [f32; 16]);

impl Transform {
    pub fn identity() -> Self {
        Self::from(&fjTransform::identity())
    }

    /// Compute transform used for vertices
    ///
    /// The returned transform is used for transforming vertices on the GPU.
    pub fn for_vertices(camera: &Camera, aspect_ratio: f64) -> Self {
        let field_of_view_in_y = camera.field_of_view_in_x() / aspect_ratio;

        let transform = camera.camera_to_model().project_to_slice(
            aspect_ratio,
            field_of_view_in_y,
            camera.near_plane(),
            camera.far_plane(),
        );

        Self(transform.map(|val| val as f32))
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera.camera_to_model().inverse().transpose();

        Self::from(&transform)
    }
}

impl From<&fjTransform> for Transform {
    fn from(other: &fjTransform) -> Self {
        let mut native = [0.0; 16];
        native.copy_from_slice(other.data());

        Self(native.map(|val| val as f32))
    }
}
