let user_guide_root;

if (!process.env.PUBLISH_MODE) {
  user_guide_root = "user_guide/templates/";
} else {
  user_guide_root = "user_guide/";
}

module.exports = {
  docs: [
    'about_parry',
    {
      'User Guide': [
        user_guide_root + 'getting_started',
        user_guide_root + 'geometric_representations',
        user_guide_root + 'bounding_volumes',
        user_guide_root + 'geometric_queries',
        user_guide_root + 'mesh_transformation',
      ],
    },
    {
      'API Documentation': [
        {
          type: 'link',
          label: 'parry2d',
          href: 'https://docs.rs/parry2d'
        },
        {
          type: 'link',
          label: 'parry3d',
          href: 'https://docs.rs/parry3d'
        },
        {
          type: 'link',
          label: 'parry2d-f64',
          href: 'https://docs.rs/parry2d-f64'
        },
        {
          type: 'link',
          label: 'parry3d-f64',
          href: 'https://docs.rs/parry3d-f64'
        },
      ],
    }
  ],
};
