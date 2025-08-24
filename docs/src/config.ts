import { defineAdditionalConfig } from 'vitepress';

export default defineAdditionalConfig({
  lang: 'en-US',
  description: '',
  themeConfig: {
    nav: [
      {
        text: 'Guide',
        link: '/guide',
        activeMatch: '/guide/',
      },
    ],

    sidebar: {
      '/guide/': {
        base: '/guide/',
        items: [{
          text: 'Guide',
          items: [
            { text: 'Chat', link: 'chat' },
            { text: 'Scripts', link: 'scripts' },
          ],
        }],
      },
    },
  },
});
