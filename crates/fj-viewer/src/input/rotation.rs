use fj_math::{Point, Transform, Vector};

use crate::camera::{Camera, FocusPoint};

pub struct Rotation {
    active: bool,
    focus_point: FocusPoint,
    active_rotation: Transform,
    base_rotation: Transform,
}

impl Rotation {
    pub fn new() -> Self {
        Self {
            active: false,
            focus_point: FocusPoint::none(),
            active_rotation: Transform::identity(),
            base_rotation: Transform::identity(),
        }
    }

    pub fn start(&mut self, camera: &Camera, focus_point: FocusPoint) {
        self.active = true;
        self.focus_point = focus_point;
        self.base_rotation = camera.rotation;
        self.active_rotation = Transform::identity();
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn apply(&mut self, diff_x: f64, diff_y: f64, camera: &mut Camera) {
        if self.active {
            let rotate_around: Vector<3> = self
                .focus_point
                .0
                .map_or(Point::origin(), |focus_point| focus_point.center)
                .coords;

            let f = 0.005;

            let angle_x = diff_y * f;
            let angle_y = diff_x * f;

            let trans = Transform::translation(rotate_around);

            let aa_x = Vector::unit_x() * angle_x;
            let aa_y = Vector::unit_y() * angle_y;
            let rot_x = Transform::rotation(aa_x);
            let rot_y = Transform::rotation(aa_y);

            self.active_rotation = self.active_rotation * rot_x * rot_y;
            camera.rotation = self.base_rotation
                * trans
                * self.active_rotation
                * trans.inverse();
        }
    }
}
