use parry3d::na::Point3;
use parry3d::*;

fn main() {
    {
        // DOCUSAURUS: convex_hull start
        let mut points = Vec::new();
        for _ in 0usize..10000 {
            points.push(rand::random::<Point3<f32>>() * 2.0f32);
        }

        let _convex_hull = transformation::convex_hull(&points[..]);
        // DOCUSAURUS: convex_hull stop
    }
}
