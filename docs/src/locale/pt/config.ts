import { defineAdditionalConfig } from 'vitepress';

export default defineAdditionalConfig({
  lang: 'pt-BR',
  description: '',
  themeConfig: {
    nav: [
      {
        text: 'Guia',
        link: '/locale/pt/guide/',
        activeMatch: '/locale/pt/guide/',
      },
    ],

    sidebar: {
      '/locale/pt/guide/': {
        base: '/locale/pt/guide/',
        items: [{
          text: 'Guia',
          items: [
            { text: 'Chat', link: 'chat' },
          ],
        }],
      },
    },
  },
});
