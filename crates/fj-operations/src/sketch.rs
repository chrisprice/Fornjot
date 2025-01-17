use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance, geometry::Surface, shape::Shape, topology::Face,
};
use fj_math::{Aabb, Point};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(&self, _: Tolerance, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        let surface = Surface::xy_plane();
        let points = self
            .to_points()
            .into_iter()
            .map(Point::from)
            .map(|point| surface.point_surface_to_model(&point));

        Face::builder(surface, &mut shape)
            .with_exterior_polygon(points)
            .build()
            .unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.to_points()
                .into_iter()
                .map(Point::from)
                .map(Point::to_xyz),
        )
    }
}
