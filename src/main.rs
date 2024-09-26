#![allow(unused)]
use std::fs::write;
use std::process::Command;
use plotters::prelude::*;
use rand::Rng;

const RANGE: f64 = 1000.0;

mod aabb;
mod point;
mod oct_tree;

fn main() {
    let boundary = aabb::AABB::new(point::Point::new(0.0, 0.0, 0.0), RANGE);
    let mut octtree = oct_tree::OctTree::new(boundary, 4);
    
    let mut points = vec![];
    
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = (rng.gen_range(-RANGE..RANGE) as f64) * rng.gen::<f64>();
        let y = (rng.gen_range(-RANGE..RANGE) as f64) * rng.gen::<f64>();
        let z = (rng.gen_range(-RANGE..RANGE) as f64) * rng.gen::<f64>();
        points.push(point::Point::new(x, y, z));
    }
    
    let start = std::time::Instant::now();

    for point in points.iter() {
        octtree.insert(point);
    }

    println!("Time elapsed: {:?}", start.elapsed());

    // query the octtree
    let range = aabb::AABB::new(point::Point::new(0.0, 0.0, 0.0), 50.0);
    let mut found_points = vec![];
    octtree.query(&range, &mut found_points);

    println!("Found points: {:?}", found_points);

    let json_file = "octtree.json";
    write(json_file, octtree.export_to_json()).expect("failed to write to json file");

    let mut child = Command::new("python")
        .arg("quadtree_plot.py")
        .arg(json_file)
        .arg(RANGE.to_string())
        .spawn()
        .expect("failed to execute quadtree_plot.py");

    // Wait for the child process to finish
    let _ = child.wait().expect("failed to wait on child process");

}
