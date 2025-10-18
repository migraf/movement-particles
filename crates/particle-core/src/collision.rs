//! Collision detection and spatial partitioning

use crate::physics::Vec2;
use std::collections::HashMap;

/// Axis-aligned bounding box
#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

/// Line segment for outline representation
#[derive(Clone, Debug)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
}

impl LineSegment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        let direction = end - start;
        let normal = Vec2::new(-direction.y, direction.x).normalize_or_zero();
        Self { start, end, normal }
    }

    /// Returns the closest point on the segment to the given point
    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        let segment = self.end - self.start;
        let point_vec = point - self.start;
        let t = point_vec.dot(segment) / segment.length_squared();
        let t_clamped = t.clamp(0.0, 1.0);
        self.start + segment * t_clamped
    }

    /// Returns the distance from a point to this segment
    pub fn distance_to(&self, point: Vec2) -> f32 {
        let closest = self.closest_point(point);
        point.distance(closest)
    }
}

/// Detected person outline for particle collision
#[derive(Clone, Debug)]
pub struct Outline {
    pub segments: Vec<LineSegment>,
    pub bounds: AABB,
    pub velocity: Vec2,
}

impl Outline {
    /// Creates an outline from a list of points
    pub fn from_points(points: Vec<Vec2>) -> Self {
        if points.is_empty() {
            return Self {
                segments: Vec::new(),
                bounds: AABB::new(Vec2::ZERO, Vec2::ZERO),
                velocity: Vec2::ZERO,
            };
        }

        let mut segments = Vec::new();
        let mut min = points[0];
        let mut max = points[0];

        for i in 0..points.len() {
            let next_i = (i + 1) % points.len();
            segments.push(LineSegment::new(points[i], points[next_i]));

            min = min.min(points[i]);
            max = max.max(points[i]);
        }

        Self {
            segments,
            bounds: AABB::new(min, max),
            velocity: Vec2::ZERO,
        }
    }

    /// Checks if a point is inside the outline
    pub fn contains(&self, point: Vec2) -> bool {
        if !self.bounds.contains(point) {
            return false;
        }

        // Ray casting algorithm
        let mut intersections = 0;
        let ray_end = Vec2::new(self.bounds.max.x + 1.0, point.y);

        for segment in &self.segments {
            if Self::ray_intersects_segment(point, ray_end, segment.start, segment.end) {
                intersections += 1;
            }
        }

        intersections % 2 == 1
    }

    fn ray_intersects_segment(ray_start: Vec2, ray_end: Vec2, seg_start: Vec2, seg_end: Vec2) -> bool {
        let r = ray_end - ray_start;
        let s = seg_end - seg_start;
        let qp = seg_start - ray_start;

        let r_cross_s = r.x * s.y - r.y * s.x;
        let qp_cross_r = qp.x * r.y - qp.y * r.x;

        if r_cross_s.abs() < 0.0001 {
            return false;
        }

        let t = (qp.x * s.y - qp.y * s.x) / r_cross_s;
        let u = qp_cross_r / r_cross_s;

        t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
    }

    /// Returns the centroid of the outline
    pub fn centroid(&self) -> Vec2 {
        if self.segments.is_empty() {
            return Vec2::ZERO;
        }

        let sum: Vec2 = self.segments.iter().map(|s| s.start).sum();
        sum / self.segments.len() as f32
    }
}

/// Spatial grid for efficient particle queries
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
    width: f32,
    height: f32,
}

impl SpatialGrid {
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            width,
            height,
        }
    }

    /// Clears all particles from the grid
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Inserts a particle into the grid
    pub fn insert(&mut self, particle_idx: usize, position: Vec2) {
        let cell = self.get_cell(position);
        self.cells.entry(cell).or_insert_with(Vec::new).push(particle_idx);
    }

    /// Returns particle indices in cells near the given position
    pub fn query_nearby(&self, position: Vec2, radius: f32) -> Vec<usize> {
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.get_cell(position);
        let mut result = Vec::new();

        for dy in -cell_radius..=cell_radius {
            for dx in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if let Some(indices) = self.cells.get(&cell) {
                    result.extend_from_slice(indices);
                }
            }
        }

        result
    }

    fn get_cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }
}

