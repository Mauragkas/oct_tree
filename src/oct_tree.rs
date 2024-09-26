use crate::{aabb::AABB, point::Point};
use serde::{Deserialize, Serialize};
use plotters::prelude::*;
use plotters::coord::Shift;

#[derive(Debug, Serialize, Deserialize)]
pub struct OctTree {
    pub boundary: AABB,
    pub capacity: usize,
    pub points: Vec<Point>,

    pub divided: bool,

    pub fne: Option<Box<OctTree>>,
    pub fnw: Option<Box<OctTree>>,
    pub fse: Option<Box<OctTree>>,
    pub fsw: Option<Box<OctTree>>,
    pub bne: Option<Box<OctTree>>,
    pub bnw: Option<Box<OctTree>>,
    pub bse: Option<Box<OctTree>>,
    pub bsw: Option<Box<OctTree>>,
}

impl OctTree {
    pub fn new(boundary: AABB, capacity: usize) -> OctTree {
        OctTree {
            boundary,
            capacity,
            points: Vec::new(),
            divided: false,
            fne: None,
            fnw: None,
            fse: None,
            fsw: None,
            bne: None,
            bnw: None,
            bse: None,
            bsw: None,
        }
    }

    pub fn insert(&mut self, point: &Point) {
        // Check if the point fits within the boundary
        if !self.boundary.contains(&point) {
            return;
        }
    
        // If there's space and we're not subdivided yet, insert the point here
        if self.points.len() < self.capacity && !self.divided {
            self.points.push(point.clone());
            return;
        }
    
        // Subdivide if necessary
        if !self.divided {
            self.subdivide();
    
            // Redistribute the points in the current node to the sub-quadrants
            let points_to_redistribute = std::mem::take(&mut self.points);
            for p in points_to_redistribute {
                self.fne.as_mut().unwrap().insert(&p);
                self.fnw.as_mut().unwrap().insert(&p);
                self.fse.as_mut().unwrap().insert(&p);
                self.fsw.as_mut().unwrap().insert(&p);
                self.bne.as_mut().unwrap().insert(&p);
                self.bnw.as_mut().unwrap().insert(&p);
                self.bse.as_mut().unwrap().insert(&p);
                self.bsw.as_mut().unwrap().insert(&p);
            }
        }
    
        // Insert the new point into the appropriate quadrant
        self.fne.as_mut().unwrap().insert(&point);
        self.fnw.as_mut().unwrap().insert(&point);
        self.fse.as_mut().unwrap().insert(&point);
        self.fsw.as_mut().unwrap().insert(&point);
        self.bne.as_mut().unwrap().insert(&point);
        self.bnw.as_mut().unwrap().insert(&point);
        self.bse.as_mut().unwrap().insert(&point);
        self.bsw.as_mut().unwrap().insert(&point);
    }    

    pub fn subdivide(&mut self) {
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let z = self.boundary.center.z;
        let half_size = self.boundary.half_size / 2.0;

        let fne = AABB::new(Point::new(x + half_size, y - half_size, z + half_size), half_size);
        let fnw = AABB::new(Point::new(x - half_size, y - half_size, z + half_size), half_size);
        let fse = AABB::new(Point::new(x + half_size, y + half_size, z + half_size), half_size);
        let fsw = AABB::new(Point::new(x - half_size, y + half_size, z + half_size), half_size);
        let bne = AABB::new(Point::new(x + half_size, y - half_size, z - half_size), half_size);
        let bnw = AABB::new(Point::new(x - half_size, y - half_size, z - half_size), half_size);
        let bse = AABB::new(Point::new(x + half_size, y + half_size, z - half_size), half_size);
        let bsw = AABB::new(Point::new(x - half_size, y + half_size, z - half_size), half_size);

        self.fne = Some(Box::new(OctTree::new(fne, self.capacity)));
        self.fnw = Some(Box::new(OctTree::new(fnw, self.capacity)));
        self.fse = Some(Box::new(OctTree::new(fse, self.capacity)));
        self.fsw = Some(Box::new(OctTree::new(fsw, self.capacity)));
        self.bne = Some(Box::new(OctTree::new(bne, self.capacity)));
        self.bnw = Some(Box::new(OctTree::new(bnw, self.capacity)));
        self.bse = Some(Box::new(OctTree::new(bse, self.capacity)));
        self.bsw = Some(Box::new(OctTree::new(bsw, self.capacity)));

        self.divided = true;
    }

    pub fn query(&self, range: &AABB, found_points: &mut Vec<Point>) {
        if !self.boundary.intersects(&range) {
            return;
        }

        for point in self.points.iter() {
            if range.contains(&point) {
                found_points.push(point.clone());
            }
        }

        if self.divided {
            self.fne.as_ref().unwrap().query(&range, found_points);
            self.fnw.as_ref().unwrap().query(&range, found_points);
            self.fse.as_ref().unwrap().query(&range, found_points);
            self.fsw.as_ref().unwrap().query(&range, found_points);
            self.bne.as_ref().unwrap().query(&range, found_points);
            self.bnw.as_ref().unwrap().query(&range, found_points);
            self.bse.as_ref().unwrap().query(&range, found_points);
            self.bsw.as_ref().unwrap().query(&range, found_points);
        }
    }

    pub fn export_to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
