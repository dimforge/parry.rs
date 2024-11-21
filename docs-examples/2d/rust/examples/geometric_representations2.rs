use parry2d::na::*;
use parry2d::shape::*;
use parry2d::*;

fn main() {
    {
        // DOCUSAURUS: ball start
        let ball = Ball::new(1.0f32);
        assert!(ball.radius == 1.0);
        // DOCUSAURUS: ball stop
    }
    {
        // DOCUSAURUS: cuboid start
        let cuboid = Cuboid::new(Vector2::new(2.0f32, 1.0));

        assert!(cuboid.half_extents.x == 2.0);
        assert!(cuboid.half_extents.y == 1.0);
        // DOCUSAURUS: cuboid stop
    }
    {
        // DOCUSAURUS: capsule start
        let capsule = Capsule::new(Point2::new(0.0, 0.5), Point2::new(0.0, 1.0), 0.75);

        assert!(capsule.segment.a == Point2::new(0.0, 0.5));
        assert!(capsule.segment.b == Point2::new(0.0, 1.0));
        assert!(capsule.radius == 0.75);
        // DOCUSAURUS: capsule stop
    }
    {
        // DOCUSAURUS: ConvexPolygon start
        let points = [
            Point2::new(-1.0f32, 1.0),
            Point2::new(-0.5, -0.5),
            Point2::new(0.0, 0.5),
            Point2::new(0.5, -0.5),
            Point2::new(1.0, 1.0),
        ];

        let convex =
            ConvexPolygon::from_convex_hull(&points).expect("Convex hull computation failed.");
        assert!(convex.points().len() == 4); // The convex hull has only 4 vertices.
                                             // DOCUSAURUS: ConvexPolygon stop
    }
    {
        // DOCUSAURUS: ConvexPolyline start
        let points = vec![
            Point2::new(-1.0f32, 1.0),
            Point2::new(-0.5, -0.5),
            Point2::new(0.5, -0.5),
            Point2::new(1.0, 1.0),
        ];

        let convex = ConvexPolygon::from_convex_polyline(points).expect("Invalid convex polygon.");
        assert!(convex.points().len() == 4);
        // DOCUSAURUS: ConvexPolyline stop
    }
    {
        // DOCUSAURUS: compound start
        // Delta transformation matrices.
        let delta1 = Isometry2::new(Vector2::new(0.0f32, -1.5), na::zero());
        let delta2 = Isometry2::new(Vector2::new(-1.5f32, 0.0), na::zero());
        let delta3 = Isometry2::new(Vector2::new(1.5f32, 0.0), na::zero());

        // 1) Initialize the shape list.
        let mut shapes = Vec::new();
        let horizontal_box = SharedShape::new(Cuboid::new(Vector2::new(1.5f32, 0.25)));
        let vertical_box = SharedShape::new(Cuboid::new(Vector2::new(0.25f32, 1.5)));

        shapes.push((delta1, horizontal_box));
        shapes.push((delta2, vertical_box.clone()));
        shapes.push((delta3, vertical_box));

        // 2) Create the compound shape.
        let compound = Compound::new(shapes);

        assert!(compound.shapes().len() == 3);
        // DOCUSAURUS: compound stop
    }
    {
        // DOCUSAURUS: polyline start
        let points = vec![
            Point2::new(0.0, 1.0),
            Point2::new(-1.0, -1.0),
            Point2::new(0.0, -0.5),
            Point2::new(1.0, -1.0),
            Point2::new(0.0, 1.0), // This forms a loop.
        ];

        // Build the polyline.
        let polyline = Polyline::new(points, None);

        assert!(polyline.vertices().len() == 4);
        // DOCUSAURUS: polyline stop
    }
}
