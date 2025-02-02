import React from 'react';
import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './styles.module.css';

const features = [
  {
    title: <>Complex shapes</>,
    imageUrl: 'img/feature_complex_shapes.svg',
    description: (
      <>
        Supports 2D and 3D shapes, from simple spheres to arbitrary
        triangle meshes, heightfields, convex polyhedra.
      </>
    ),
  },
  {
    title: <>Bounding volumes</>,
    imageUrl: 'img/feature_bounding_volumes.svg',
    description: (
      <>
        Bound complex shapes with simpler ones like AABB and bounding spheres
        to perform efficient conservative geometric queries.
      </>
    ),
  },
  {
    title: <>Ray casting</>,
    imageUrl: 'img/feature_ray_casting.svg',
    description: (
      <>
        Compute intersections between a ray and any shape. Typical
        use-case include object selection and rendering (ray-tracing).
      </>
    ),
  },
  {
    title: <>Point projection</>,
    imageUrl: 'img/feature_point_projection.svg',
    description: (
      <>
        Test a point for containment, compute distances to a point, or project it
        on any shape.
      </>
    ),
  },
  {
    title: <>Contact points</>,
    imageUrl: 'img/feature_contact_points.svg',
    description: (
      <>
        Find the closest points between objects in close proximity.  If they are
        penetrating, the minimal translational distance can be obtained as well!
      </>
    ),
  },
  {
    title: <>Sweep tests</>,
    imageUrl: 'img/feature_time_of_impact.svg',
    description: (
      <>
        Compute the time it would take for two moving shapes to start being in
        contact.
      </>
    ),
  },
  {
    title: <>Distance computation</>,
    imageUrl: 'img/feature_smallest_distance.svg',
    description: (
      <>
        Compute the global minimal distance between two shapes.
      </>
    ),
  },
  {
    title: <>Convex hulls and decompositions</>,
    imageUrl: 'img/hacd_small.png',
    description: (
      <>
        Generate triangle meshes from smooth objects, compute their convex hull, or
        decompose them approximately into their convex components.            </>
    ),
  },
  {
    title: <>Forever free and Open-Source</>,
    imageUrl: 'img/undraw_join_of2w.svg',
    description: (
      <>
        Built with a FOSS mindset, we aim to empower the Rust and web communities
        with an efficient collision-detection framework.
      </>
    ),
  },
];

function Feature({ imageUrl, title, description }) {
  const imgUrl = useBaseUrl(imageUrl);
  return (
    <div className={clsx('col col--4', styles.feature)}>
      {imgUrl && (
        <div className="text--center">
          <img className={styles.featureImage} src={imgUrl} alt={title} />
        </div>
      )}
      <h3>{title}</h3>
      <p>{description}</p>
    </div>
  );
}

function Home() {
  const context = useDocusaurusContext();
  const { siteConfig = {} } = context;
  return (
    <Layout
      title={`${siteConfig.title} collision-detection library`}
      description="Fast and cross-platform collision-detection library">
      <header className={clsx('hero hero--primary', styles.heroBanner)}>
        <div className="container">
          <div className="">
            <img src="img/logo_parry.svg" width="30%" alt="Project Logo" />
          </div>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link
              className={clsx(
                'button button--outline button--lg --ifm-color-prim force-border', /*button--secondary*/
                styles.getStarted,
              )}
              to={useBaseUrl('docs/user_guide/getting_started')}>
              Get Started
            </Link>
          </div>
        </div>
      </header>
      <main>
        {features && features.length > 0 && (
          <section className={styles.features}>
            <div className="container">
              <div className="row">
                {features.map((props, idx) => (
                  <Feature key={idx} {...props} />
                ))}
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  );
}

export default Home;
