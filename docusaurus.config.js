const math = require('remark-math')
const katex = require('rehype-katex')

const config = {
  title: 'Parry',
  tagline: '2D and 3D collision-detection library for the Rust programming language.',
  url: 'https://parry.rs',
  baseUrl: '/',
  // Not sure why some links are detected are broken,
  // or why setting `throw` results in a fail compilation where https://github.com/dimforge/nalgebra.rs/pull/88 didn´t.
  onBrokenLinks: 'warn',
  favicon: 'img/favicon.png',
  organizationName: 'dimforge', // Usually your GitHub org/user name.
  projectName: 'parry', // Usually your repo name.
  themeConfig: {
    prism: {
      theme: require('prism-react-renderer').themes.github,
      additionalLanguages: ['toml', 'rust'],
    },
    // announcementBar: {
    //   id: 'supportus',
    //   content:
    //     '⭐️ If you like Parry, support us on <a target="_blank" rel="noopener noreferrer" href="https://github.com/sponsors/dimforge">GitHub Sponsor</a>! ⭐️',
    // },
    navbar: {
      title: 'Parry',
      logo: {
        alt: 'Parry Logo',
        src: 'img/logo_parry_wo_text.svg',
      },
      hideOnScroll: true,
      items: [
        {
          to: 'docs/',
          activeBasePath: 'docs',
          label: 'Docs',
          position: 'left',
        },
        {
          to: '/community',
          position: 'left',
          activeBaseRegex: `/community/`,
          label: 'Community',
        },
        {
          href: 'https://dimforge.com/blog',
          label: 'Blog',
          position: 'left',
        },
        // {to: 'blog', label: 'Blog', position: 'left'},
        {
          value: '<a class="header-button-donate" href="https://github.com/sponsors/dimforge" target="_blank" rel="noopener noreferrer">Donate ♥</a>',
          position: 'right',
          className: "header-button-donate",
          type: 'html',
        },
        {
          href: 'https://dimforge.com',
          label: 'Dimforge',
          position: 'right',
        },
        {
          href: 'https://github.com/dimforge/parry',
          position: 'right',
          className: 'header-github-link',
          'aria-label': 'GitHub repository',
        },
      ],
    },
    footer: {
      style: 'dark',
      logo: {
        alt: 'Dimforge EURL Logo',
        src: 'https://www.dimforge.com/img/logo/logo_dimforge_small_rect.svg',
        href: 'https://www.dimforge.com/'
      },
      copyright: `Built by <a href="https://www.dimforge.com">Dimforge, EURL</a>.`,
      links: [
        {
          title: 'Resources',
          items: [
            {
              label: 'Documentation',
              to: 'docs/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Stack Overflow',
              href: 'https://stackoverflow.com/questions/tagged/parry',
            },
            {
              label: 'Discord',
              href: 'https://discord.gg/vt9DJSW',
            },
            // {
            //   label: 'Twitter',
            //   href: 'https://twitter.com/dimforge',
            // },
          ],
        },
        {
          title: 'More',
          items: [
            // {
            //   label: 'Blog',
            //   to: 'blog',
            // },
            {
              label: 'GitHub',
              href: 'https://github.com/dimforge/parry',
            },
          ],
        },
      ],
      // copyright: `Copyright © ${new Date().getFullYear()} Dimforge EURL. Website built with Docusaurus.`,
    },
  },
  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'community',
        path: 'community',
        routeBasePath: 'community',
        sidebarPath: require.resolve('./sidebar_community.js'),
        showLastUpdateTime: false,
      }
    ],
  ],
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebar_docs.js'),
          showLastUpdateTime: false,
          remarkPlugins: [math],
          rehypePlugins: [katex],
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://github.com/dimforge/parry.rs/edit/master/website/blog/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
  stylesheets: [
    'https://cdn.jsdelivr.net/npm/katex@0.11.0/dist/katex.min.css'
  ]
};

export default config; 