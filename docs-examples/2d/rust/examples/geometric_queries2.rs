use approx::relative_eq;
use parry2d::na::*;
use parry2d::query::ClosestPoints;
use parry2d::query::ShapeCastOptions;
use parry2d::shape::*;
use parry2d::*;

fn main() {
    {
        // DOCUSAURUS: closest_points start
        let cuboid = Cuboid::new(Vector2::new(1.0, 1.0));
        let ball = Ball::new(1.0);
        let margin = 1.0;

        let cuboid_pos = na::one();
        let ball_pos_intersecting = Isometry2::new(Vector2::new(1.0, 1.0), na::zero());
        let ball_pos_within_margin = Isometry2::new(Vector2::new(2.0, 2.0), na::zero());
        let ball_pos_disjoint = Isometry2::new(Vector2::new(3.0, 3.0), na::zero());

        let prox_intersecting =
            query::closest_points(&ball_pos_intersecting, &ball, &cuboid_pos, &cuboid, margin);
        let prox_within_margin =
            query::closest_points(&ball_pos_within_margin, &ball, &cuboid_pos, &cuboid, margin);
        let prox_disjoint =
            query::closest_points(&ball_pos_disjoint, &ball, &cuboid_pos, &cuboid, margin);

        assert_eq!(prox_intersecting, Ok(ClosestPoints::Intersecting));
        { /* TODO: The following probably doesn´t work */ }
        assert!(matches!(
            prox_within_margin,
            Ok(ClosestPoints::WithinMargin(_, _))
        ));
        assert!(matches!(prox_disjoint, Ok(ClosestPoints::Disjoint)));
        // DOCUSAURUS: closest_points stop
    }
    {
        // DOCUSAURUS: distance start
        /*
         * Initialize the shapes.
         */
        let cuboid = Cuboid::new(Vector2::new(1.0, 1.0));
        let ball = Ball::new(1.0);

        let cuboid_pos = na::one();
        let ball_pos_intersecting = Isometry2::new(Vector2::y(), na::zero());
        let ball_pos_disjoint = Isometry2::new(Vector2::y() * 3.0, na::zero());

        let dist_intersecting =
            query::distance(&ball_pos_intersecting, &ball, &cuboid_pos, &cuboid);
        let dist_disjoint =
            query::distance(&ball_pos_disjoint, &ball, &cuboid_pos, &cuboid).unwrap();

        assert_eq!(dist_intersecting, Ok(0.0));
        assert!(relative_eq!(dist_disjoint, 1.0, epsilon = 1.0e-7));
        // DOCUSAURUS: distance stop
    }
    #[rustfmt::skip]
    pub fn no_fmt()
    {
        // DOCUSAURUS: contact start
        let cuboid     = Cuboid::new(Vector2::new(1.0, 1.0));
        let ball       = Ball::new(1.0);
        let prediction = 1.0;

        let cuboid_pos             = na::one();
        let ball_pos_penetrating   = Isometry2::new(Vector2::new(1.0, 1.0), na::zero());
        let ball_pos_in_prediction = Isometry2::new(Vector2::new(2.0, 2.0), na::zero());
        let ball_pos_too_far       = Isometry2::new(Vector2::new(3.0, 3.0), na::zero());

        let ctct_penetrating = query::contact(&ball_pos_penetrating, &ball,
                                            &cuboid_pos,           &cuboid,
                                            prediction);
        let ctct_in_prediction = query::contact(&ball_pos_in_prediction, &ball,
                                                &cuboid_pos,             &cuboid,
                                                prediction);
        let ctct_too_far = query::contact(&ball_pos_too_far, &ball,
                                        &cuboid_pos,       &cuboid,
                                        prediction);

        assert!(ctct_penetrating.unwrap().unwrap().dist > 0.0);
        assert!(ctct_in_prediction.unwrap().unwrap().dist < 0.0);
        assert_eq!(ctct_too_far, Ok(None));
        // DOCUSAURUS: contact stop

        {
            // DOCUSAURUS: cast_shapes start
            let cuboid = Cuboid::new(Vector2::new(1.0, 1.0));
            let ball = Ball::new(1.0);
    
            let cuboid_pos = na::one();
            let ball_pos_intersecting = Isometry2::new(Vector2::new(1.0, 1.0), na::zero());
            let ball_pos_will_touch = Isometry2::new(Vector2::new(2.0, 2.0), na::zero());
            let ball_pos_wont_touch = Isometry2::new(Vector2::new(3.0, 3.0), na::zero());
    
            let box_vel1 = Vector2::new(-1.0, 1.0);
            let box_vel2 = Vector2::new(1.0, 1.0);
    
            let ball_vel1 = Vector2::new(2.0, 2.0);
            let ball_vel2 = Vector2::new(-0.5, -0.5);
    
            let shape_cast_options = ShapeCastOptions::default();
    
            let result_intersecting = query::cast_shapes(
                &ball_pos_intersecting,
                &ball_vel1,
                &ball,
                &cuboid_pos,
                &box_vel1,
                &cuboid,
                shape_cast_options,
            )
            .unwrap();
            let result_will_touch = query::cast_shapes(
                &ball_pos_will_touch,
                &ball_vel2,
                &ball,
                &cuboid_pos,
                &box_vel2,
                &cuboid,
                shape_cast_options,
            )
            .unwrap();
            let result_wont_touch = query::cast_shapes(
                &ball_pos_wont_touch,
                &ball_vel1,
                &ball,
                &cuboid_pos,
                &box_vel1,
                &cuboid,
                shape_cast_options,
            )
            .unwrap();
    
            assert_eq!(result_intersecting.unwrap().time_of_impact, 0.0);
            assert!(result_will_touch.unwrap().time_of_impact > 0.0);
            assert!(result_wont_touch.is_none());
            // DOCUSAURUS: cast_shapes stop
        }
    }
    no_fmt();
}
