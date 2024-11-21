use parry2d::bounding_volume::BoundingVolume;
use parry2d::na::Isometry2;
use parry2d::na::Vector2;
use parry2d::shape::Ball;
use parry2d::shape::Cuboid;
use parry2d::shape::Shape;
use parry2d::*;

fn main() {
    {
        // DOCUSAURUS: Bounding start
        /*
         * Initialize the shapes.
         */
        let cube1 = Cuboid::new(Vector2::repeat(0.5));
        let cube2 = Cuboid::new(Vector2::new(1.0, 0.5));

        let cube1_pos = Isometry2::new(Vector2::y(), na::zero()); // 1.0 along the `y` axis.
        let cube2_pos = Isometry2::identity(); // Identity transformation.

        /*
         * Compute their bounding spheres.
         */
        let bounding_sphere_cube1 = cube1.compute_bounding_sphere(&cube1_pos);
        let bounding_sphere_cube2 = cube2.compute_bounding_sphere(&cube2_pos);

        // Merge the two spheres.
        let bounding_bounding_sphere = bounding_sphere_cube1.merged(&bounding_sphere_cube2);

        // Enlarge the cube2 bounding sphere.
        let loose_bounding_sphere_cube2 = bounding_sphere_cube2.loosened(1.0);

        // Intersection and inclusion tests.
        assert!(bounding_sphere_cube1.intersects(&bounding_sphere_cube2));
        assert!(bounding_bounding_sphere.contains(&bounding_sphere_cube1));
        assert!(bounding_bounding_sphere.contains(&bounding_sphere_cube2));
        assert!(!bounding_sphere_cube2.contains(&bounding_bounding_sphere));
        assert!(!bounding_sphere_cube1.contains(&bounding_bounding_sphere));
        assert!(loose_bounding_sphere_cube2.contains(&bounding_sphere_cube2));
        // DOCUSAURUS: Bounding stop
    }
    {
        // DOCUSAURUS: Aabb start
        /*
         * Initialize the shapes.
         */
        let ball1 = Ball::new(0.5);
        let ball2 = Ball::new(1.0);

        let ball1_pos = Isometry2::new(Vector2::y(), na::zero()); // 1.0 along the `y` axis.
        let ball2_pos = Isometry2::identity(); // Identity matrix.

        /*
         * Compute their axis-aligned bounding boxes.
         */
        let aabb_ball1 = ball1.compute_aabb(&ball1_pos);
        let aabb_ball2 = ball2.compute_aabb(&ball2_pos);

        // Merge the two boxes.
        let bounding_aabb = aabb_ball1.merged(&aabb_ball2);

        // Enlarge the ball2 aabb.
        let loose_aabb_ball2 = aabb_ball2.loosened(1.0);

        // Intersection and inclusion tests.
        assert!(aabb_ball1.intersects(&aabb_ball2));
        assert!(bounding_aabb.contains(&aabb_ball1));
        assert!(bounding_aabb.contains(&aabb_ball2));
        assert!(!aabb_ball2.contains(&bounding_aabb));
        assert!(!aabb_ball1.contains(&bounding_aabb));
        assert!(loose_aabb_ball2.contains(&aabb_ball2));
        // DOCUSAURUS: Aabb stop
    }
}
