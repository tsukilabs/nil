import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'Call of Nil',
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

  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
    },
    'locale/pt': {
      label: 'Português',
      lang: 'pt-BR',
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
  },
});
