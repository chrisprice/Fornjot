use fj_math::{Point, Scalar, Transform, Vector};

use crate::{
    camera::{Camera, FocusPoint},
    screen::{Position, Size},
};

use super::handler::Behavior;

pub struct Movement {
    active: bool,
    focus_point: Option<FocusPoint>,
    cursor: Option<Position>,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            active: false,
            focus_point: None,
            cursor: None,
        }
    }

    pub fn start(
        &mut self,
        focus_point: Option<FocusPoint>,
        cursor: Option<Position>,
    ) {
        self.active = true;
        self.focus_point = focus_point;
        self.cursor = cursor;
    }

    pub fn stop(&mut self) {
        self.active = false;
        self.focus_point = None;
    }

    pub fn apply(
        &mut self,
        cursor: Option<Position>,
        camera: &mut Camera,
        size: Size,
    ) {
        if let (Some(previous), Some(cursor)) = (self.cursor, cursor) {
            let previous = camera.cursor_to_model_space(previous, size);
            let cursor = camera.cursor_to_model_space(cursor, size);

            if let Some(focus_point) = self.focus_point {
                let d1 = Point::distance(&camera.position(), &cursor);
                let d2 =
                    Point::distance(&camera.position(), &focus_point.center);

                let diff = (cursor - previous) * d2 / d1;
                let offset = camera.camera_to_model().transform_vector(&diff);

                camera.translation = camera.translation
                    * Transform::translation(Vector::from([
                        offset.x,
                        offset.y,
                        Scalar::ZERO,
                    ]));
            }
        }

        self.cursor = cursor;
    }
}

impl Behavior for Movement {
    fn is_active(&self) -> bool {
        self.active
    }
}
