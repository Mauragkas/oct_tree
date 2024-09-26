use crate::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AABB {
    pub center: Point,
    pub half_size: f64,
}

impl AABB {
    pub fn new(center: Point, half_size: f64) -> AABB {
        AABB { center, half_size }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return self.center.x - self.half_size <= point.x
            && point.x <= self.center.x + self.half_size
            && self.center.y - self.half_size <= point.y
            && point.y <= self.center.y + self.half_size
            && self.center.z - self.half_size <= point.z
            && point.z <= self.center.z + self.half_size;
    }

    pub fn intersects(&self, range: &AABB) -> bool {
        return !(range.center.x - range.half_size > self.center.x + self.half_size
            || range.center.x + range.half_size < self.center.x - self.half_size
            || range.center.y - range.half_size > self.center.y + self.half_size
            || range.center.y + range.half_size < self.center.y - self.half_size
            || range.center.z - range.half_size > self.center.z + self.half_size
            || range.center.z + range.half_size < self.center.z - self.half_size);
    }
}
