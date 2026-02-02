import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'Call of Nil',
  description: '',
  srcDir: 'src',
  outDir: 'dist',
  cleanUrls: true,
  metaChunk: true,
  lastUpdated: false,

  head: [
    ['link', { rel: 'icon', href: '/favicon.png' }],
  ],

  sitemap: {
    hostname: 'https://nil.dev.br',
  },

  vite: {
    build: {
      emptyOutDir: true,
      minify: true,
      target: 'baseline-widely-available',
    },
  },

  themeConfig: {
    logo: '/favicon.png',

    outline: {
      level: [2, 3],
    },

    search: {
      provider: 'local',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/tsukilabs/nil' },
      { icon: 'discord', link: 'https://discord.gg/c2kvRWJSfz' },
    ],

    nav: [
      {
        text: 'Guide',
        link: '/guide/',
        activeMatch: '^/guide/',
      },
    ],

    sidebar: {
      '/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Introduction', link: '/introduction' },
            { text: 'Download', link: '/download' },
          ],
        },
        {
          text: 'Guide',
          items: [
            { text: 'Chat', link: '/guide/chat' },
          ],
        },
      ],
    },
  },
});
