import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'xtrace',
  description: 'AI Observability — Traces, Metrics & Spans for LLM and Agent Workflows',
  lang: 'en-US',
  cleanUrls: true,

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo.svg' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;700&display=swap', rel: 'stylesheet' }],
  ],

  themeConfig: {
    logo: '/logo.svg',
    siteTitle: 'xtrace',

    nav: [
      { text: 'Guide', link: '/guide/getting-started', activeMatch: '/guide/' },
      { text: 'API', link: '/api/rest-api', activeMatch: '/api/' },
      { text: 'SDK', link: '/sdk/rust-client', activeMatch: '/sdk/' },
      { text: 'Integrations', link: '/integrations/nebula', activeMatch: '/integrations/' },
      {
        text: 'v0.0.14',
        items: [
          { text: 'Changelog', link: 'https://github.com/lipish/xtrace/releases' },
          { text: 'crates.io', link: 'https://crates.io/crates/xtrace' },
        ],
      },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Configuration', link: '/guide/configuration' },
          ],
        },
        {
          text: 'Advanced',
          items: [
            { text: 'Development', link: '/guide/development' },
            { text: 'Deployment', link: '/guide/deployment' },
          ],
        },
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'REST API', link: '/api/rest-api' },
            { text: 'Metrics API', link: '/api/metrics-api' },
            { text: 'OTLP Ingestion', link: '/api/otlp' },
          ],
        },
      ],
      '/sdk/': [
        {
          text: 'SDK',
          items: [
            { text: 'Rust Client', link: '/sdk/rust-client' },
            { text: 'Python SDK', link: '/sdk/python-sdk' },
          ],
        },
      ],
      '/integrations/': [
        {
          text: 'Integrations',
          items: [
            { text: 'Nebula', link: '/integrations/nebula' },
            { text: 'Langfuse', link: '/integrations/langfuse' },
            { text: 'tracing (Rust)', link: '/integrations/tracing' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/lipish/xtrace' },
    ],

    editLink: {
      pattern: 'https://github.com/lipish/xtrace/edit/main/www/:path',
      text: 'Edit this page on GitHub',
    },

    footer: {
      message: 'Released under the MIT License.',
      copyright: '© 2025-present xtrace contributors',
    },

    search: {
      provider: 'local',
    },
  },
})
