use parry3d::na::Isometry3;
use parry3d::na::Point3;
use parry3d::na::Vector3;
use parry3d::shape::*;
use parry3d::*;

fn main() {
    {
        // DOCUSAURUS: ball start
        let ball = Ball::new(1.0f32);
        assert!(ball.radius == 1.0);
        // DOCUSAURUS: ball stop
    }
    {
        // DOCUSAURUS: cuboid start
        let cuboid = Cuboid::new(Vector3::new(2.0f32, 1.0, 3.0));

        assert!(cuboid.half_extents.x == 2.0);
        assert!(cuboid.half_extents.y == 1.0);
        assert!(cuboid.half_extents.z == 3.0);
        // DOCUSAURUS: cuboid stop
    }

    {
        // DOCUSAURUS: cylinder start
        let cylinder = Cylinder::new(0.5f32, 1.0);

        assert!(cylinder.half_height == 0.5);
        assert!(cylinder.radius == 1.0);
        // DOCUSAURUS: cylinder stop
    }
    {
        // DOCUSAURUS: cone start
        let cone = Cone::new(0.5f32, 0.75);

        assert!(cone.half_height == 0.5);
        assert!(cone.radius == 0.75);
        // DOCUSAURUS: cone stop
    }
    {
        // DOCUSAURUS: capsule start
        let capsule = Capsule::new(Point3::new(0.0, 0.5, 0.0), Point3::new(0.0, 1.0, 0.0), 0.75);

        assert!(capsule.segment.a == Point3::new(0.0, 0.5, 0.0));
        assert!(capsule.segment.b == Point3::new(0.0, 1.0, 0.0));
        assert!(capsule.radius == 0.75);
        // DOCUSAURUS: capsule stop
    }
    #[rustfmt::skip]
    pub fn no_fmt() {
        {
            // DOCUSAURUS: ConvexPolyhedron start
            let points = [
                Point3::new(0.0f32, 0.0, 1.0),
                Point3::new(0.0, 0.0, -1.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(0.0, -1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0)
            ];
            
            let convex = ConvexPolyhedron::from_convex_hull(&points).expect("Convex hull computation failed.");
            assert!(convex.points().len() == 6); // The convex hull has only 6 vertices.
            // DOCUSAURUS: ConvexPolyhedron stop
        }
        {
            // DOCUSAURUS: ConvexMesh start
            let points = vec![
                Point3::new(0.0f32, 0.0, 1.0),
                Point3::new(0.0, 0.0, -1.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(0.0, -1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ];
    
            let indices = vec![
                [0, 4, 2], [0, 3, 4], [5, 0, 2], [5, 3, 0], [1, 5, 2], [1, 3, 5], [4, 1, 2], [4, 3, 1],
            ];
    
            let convex =
                ConvexPolyhedron::from_convex_mesh(points, &indices).expect("Invalid convex shape.");
            assert!(convex.points().len() == 6);
            // DOCUSAURUS: ConvexMesh stop
        }
    }
    {
        use parry3d::shape::Compound;
        // DOCUSAURUS: compound start
        // Delta transformation matrices.
        let delta1 = Isometry3::new(Vector3::new(0.0f32, -1.5, 0.0), na::zero());
        let delta2 = Isometry3::new(Vector3::new(-1.5f32, 0.0, 0.0), na::zero());
        let delta3 = Isometry3::new(Vector3::new(1.5f32, 0.0, 0.0), na::zero());

        // 1) Initialize the shape list.
        let mut shapes = Vec::new();
        let horizontal_box = SharedShape::new(Cuboid::new(Vector3::new(1.5f32, 0.25, 0.25)));
        let vertical_box = SharedShape::new(Cuboid::new(Vector3::new(0.25f32, 1.5, 0.25)));

        shapes.push((delta1, horizontal_box));
        shapes.push((delta2, vertical_box.clone()));
        shapes.push((delta3, vertical_box));

        // 2) Create the compound shape.
        let compound = Compound::new(shapes);

        assert!(compound.shapes().len() == 3)
        // DOCUSAURUS: compound stop
    }
    {
        // DOCUSAURUS: polyline start
        let points = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(0.0, -0.5, 0.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(0.0, 1.0, 0.0), // This forms a loop.
        ];

        // Build the polyline.
        let polyline = Polyline::new(points, None);

        assert!(polyline.vertices().len() == 5);
        // DOCUSAURUS: polyline stop
    }
    {
        use parry3d::shape::TriMesh;
        // DOCUSAURUS: trimesh start
        let points = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, -0.5, 0.0),
            Point3::new(0.0, -0.5, -1.0),
            Point3::new(1.0, -0.5, 0.0),
        ];

        let indices = vec![[0u32, 1, 2], [0, 2, 3], [0, 3, 1]];

        // Build the mesh.
        let mesh = TriMesh::new(points, indices);

        assert!(mesh.vertices().len() == 4);
        // DOCUSAURUS: trimesh stop
    }
    no_fmt();
}
