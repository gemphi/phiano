import type { PhiDocSiteConfig } from '@phiace/phidoc';

const config: PhiDocSiteConfig = {
  title: 'Phiano',
  description: 'Phase Instrument for Intelligence: a phase-manifold approach to language, memory, and learning.',
  version: '0.1.0',
  brandId: 'phiano',
  docs: {
    path: '../docs',
    routeBasePath: 'docs',
  },
  home: {
    title: 'Phase Instrument for Intelligence',
    tagline: 'A Rust-native phase-manifold engine for language modeling, memory layers, and native learning.',
    actions: [
      { label: 'Get Started', href: '/docs/getting-started', variant: 'primary' },
      { label: 'Architecture', href: '/docs/architecture-overview', variant: 'outline' },
      { label: 'API Reference', href: '/docs/api-reference', variant: 'secondary' },
    ],
    features: [
      {
        title: 'Phase Manifold',
        description: 'Tokens, memory, and context modeled as coupled oscillators on a phase manifold.',
        href: '/docs/phase-manifold',
      },
      {
        title: 'Native Learning',
        description: 'Continuous Kuramoto-style learning cycles without heavyweight transformer infrastructure.',
        href: '/docs/learning-cycle',
      },
      {
        title: 'Rust-Native Core',
        description: 'Deterministic, inspectable engine with REPL, scoring, and composition pipelines.',
        href: '/docs/file-map',
      },
    ],
  },
  nav: [
    { label: 'Docs', href: '/docs/readme' },
    { label: 'Papers', href: '/docs/papers' },
    { label: 'API', href: '/docs/api-reference' },
    { label: 'GitHub', href: 'https://github.com/phiace/phiano', external: true },
  ],
  sidebar: {
    mode: 'auto',
  },
  layout: {
    navbarSticky: true,
    navbarVariant: 'default',
    sidebarCollapsed: false,
  },
};

export default config;
