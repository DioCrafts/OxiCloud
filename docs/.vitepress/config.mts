import { defineConfig } from "vitepress";

export default defineConfig({
  title: "OxiCloud",
  description: "Self-hosted cloud storage, calendar & contacts — blazingly fast",

  base: "/OxiCloud/",

  sitemap: {
    hostname: "https://diocrafts.github.io/OxiCloud",
    lastmodDateOnly: false,
  },

  markdown: {
    image: {
      lazyLoading: true,
    },
  },

  lastUpdated: true,

  ignoreDeadLinks: [
    /^https?:\/\/localhost/,
  ],

  locales: {
    root: {
      label: "English",
      lang: "en",
    },
    es: {
      label: "Español",
      lang: "es",
      link: "/es/",
      title: "OxiCloud",
      description: "Almacenamiento en la nube autoalojado, calendario y contactos — increíblemente rápido",
      themeConfig: {
        nav: [
          { text: "Inicio", link: "/es/" },
          { text: "Guía", link: "/es/guide/" },
          { text: "Configuración", link: "/es/config/" },
          { text: "FAQ", link: "/es/faq" },
        ],
        editLink: {
          pattern: "https://github.com/DioCrafts/OxiCloud/tree/main/docs/:path",
          text: "Editar esta página en GitHub",
        },
        sidebar: {
          "/es/": [
            {
              text: "Introducción",
              items: [
                { text: "¿Qué es OxiCloud?", link: "/es/guide/" },
                { text: "Inicio Rápido", link: "/es/guide/installation" },
              ],
            },
            {
              text: "Configuración",
              items: [
                { text: "Despliegue & Docker", link: "/es/config/deployment" },
                { text: "Variables de Entorno", link: "/es/config/env" },
                { text: "OIDC / SSO", link: "/es/config/oidc" },
                { text: "WOPI (Office)", link: "/es/config/wopi" },
              ],
            },
            {
              text: "Características",
              items: [
                { text: "WebDAV", link: "/es/guide/webdav" },
                { text: "CalDAV & CardDAV", link: "/es/guide/caldav-carddav" },
                { text: "Subida Chunked", link: "/es/guide/chunked-uploads" },
                { text: "Deduplicación", link: "/es/guide/deduplication" },
                { text: "Búsqueda", link: "/es/guide/search" },
                { text: "Papelera", link: "/es/guide/trash" },
              ],
            },
            {
              text: "Arquitectura",
              items: [
                { text: "Arquitectura Interna", link: "/es/architecture/" },
                { text: "Caché", link: "/es/architecture/caching" },
                { text: "Cuotas de Almacenamiento", link: "/es/architecture/storage-quotas" },
              ],
            },
            { text: "FAQ", link: "/es/faq" },
          ],
        },
      },
    },
  },

  head: [
    ["link", { rel: "icon", href: "/OxiCloud/logo.svg" }],
  ],

  themeConfig: {
    logo: "/logo.svg",

    search: {
      provider: "local",
    },

    editLink: {
      pattern: "https://github.com/DioCrafts/OxiCloud/tree/main/docs/:path",
      text: "Edit this page on GitHub",
    },

    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/" },
      { text: "Configuration", link: "/config/" },
      { text: "FAQ", link: "/faq" },
    ],

    sidebar: {
      "/": [
        {
          text: "Introduction",
          items: [
            { text: "What is OxiCloud?", link: "/guide/" },
            { text: "Quick Start", link: "/guide/installation" },
          ],
        },
        {
          text: "Configuration",
          items: [
            { text: "Deployment & Docker", link: "/config/deployment" },
            { text: "Environment Variables", link: "/config/env" },
            { text: "OIDC / SSO", link: "/config/oidc" },
            { text: "WOPI (Office Editing)", link: "/config/wopi" },
          ],
        },
        {
          text: "Features",
          items: [
            { text: "WebDAV", link: "/guide/webdav" },
            { text: "CalDAV & CardDAV", link: "/guide/caldav-carddav" },
            { text: "Chunked Uploads", link: "/guide/chunked-uploads" },
            { text: "Deduplication", link: "/guide/deduplication" },
            { text: "Search", link: "/guide/search" },
            { text: "Trash & Recycle Bin", link: "/guide/trash" },
          ],
        },
        {
          text: "Architecture",
          items: [
            { text: "Internal Architecture", link: "/architecture/" },
            { text: "Caching", link: "/architecture/caching" },
            { text: "Storage Quotas", link: "/architecture/storage-quotas" },
          ],
        },
        { text: "FAQ", link: "/faq" },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/DioCrafts/OxiCloud" },
    ],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2025-present DioCrafts",
    },
  },
});
