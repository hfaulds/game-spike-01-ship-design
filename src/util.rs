use crate::prelude::*;

struct Ray {
    ray: bevy::math::Ray,
}

impl Ray {
    /// Returns the distance to the plane if the ray intersects it.
    pub fn intersect_plane(&self, plane_origin: Vec3, plane_normal: Vec3) -> Option<f32> {
        let denominator = plane_normal.dot(self.ray.direction);
        if denominator.abs() > f32::EPSILON {
            let distance = (plane_origin - self.ray.origin).dot(plane_normal) / denominator;
            if distance > f32::EPSILON {
                return Some(distance);
            }
        }
        None
    }

    /// Retrieve a point at the given distance along the ray.
    pub fn get_point(&self, distance: f32) -> Vec3 {
        self.ray.origin + self.ray.direction * distance
    }
}

pub fn get_cursor_position(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec3> {
    let window = windows.get_primary().unwrap();
    let cursor_position = window.cursor_position()?;
    let (camera, camera_transform) = camera.single();
    let ray = Ray {
        ray: camera
            .viewport_to_world(camera_transform, cursor_position)
            .unwrap(),
    };

    let distance = ray.intersect_plane(Vec3::Z, Vec3::Z).unwrap();
    Some(ray.get_point(distance))
}

pub fn round_to_grid(pos: Vec2, grid_size: f32) -> Vec2 {
    Vec2::new(
        (pos.x / grid_size).round() * grid_size,
        (pos.y / grid_size).round() * grid_size,
    )
}

pub fn point_relative_to_transform(point: Vec3, transform: &GlobalTransform) -> Vec3 {
    return transform.compute_matrix().inverse().transform_point3(point);
}
