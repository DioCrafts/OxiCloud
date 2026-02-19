// OxiCloud Service Worker
const CACHE_NAME = 'oxicloud-cache-v15';
const MANIFEST_URL = '/dist/manifest.json';
const CORE_ASSETS = [
  '/',
  '/index.html',
  '/login',
  '/profile',
  '/admin',
  '/shared',
  '/locales/en.json',
  '/locales/es.json',
  '/locales/fa.json',
  '/locales/de.json',
  '/favicon.ico'
];

async function getDistAssetsFromManifest() {
  try {
    const response = await fetch(MANIFEST_URL, { cache: 'no-store' });
    if (!response.ok) return [];

    const manifest = await response.json();
    const distFiles = new Set();

    if (manifest && manifest.pages) {
      Object.values(manifest.pages).forEach((page) => {
        (page?.js || []).forEach((url) => {
          if (typeof url === 'string' && url.startsWith('/dist/')) distFiles.add(url);
        });
        (page?.css || []).forEach((url) => {
          if (typeof url === 'string' && url.startsWith('/dist/')) distFiles.add(url);
        });
      });
    }

    if (manifest && Array.isArray(manifest.chunks)) {
      manifest.chunks.forEach((chunk) => {
        if (chunk && typeof chunk.file === 'string' && chunk.file.startsWith('/dist/')) {
          distFiles.add(chunk.file);
        }
      });
    }

    return Array.from(distFiles);
  } catch (_error) {
    return [];
  }
}

async function buildPrecacheList() {
  const distAssets = await getDistAssetsFromManifest();
  return Array.from(new Set([...CORE_ASSETS, ...distAssets]));
}

// Install event - precache core + hashed dist assets
self.addEventListener('install', (event) => {
  event.waitUntil((async () => {
    const assets = await buildPrecacheList();
    const cache = await caches.open(CACHE_NAME);
    await cache.addAll(assets);
    await self.skipWaiting();
  })());
});

// Activate event - clean old caches
self.addEventListener('activate', (event) => {
  event.waitUntil((async () => {
    const cacheNames = await caches.keys();
    await Promise.all(
      cacheNames
        .filter((cacheName) => cacheName !== CACHE_NAME)
        .map((cacheName) => caches.delete(cacheName))
    );
    await self.clients.claim();
  })());
});

// Fetch strategies
self.addEventListener('fetch', (event) => {
  const request = event.request;
  if (request.method !== 'GET') return;

  const url = new URL(request.url);

  // Don't intercept API requests
  if (url.pathname.startsWith('/api/')) return;

  // Hashed dist assets: cache-first
  if (url.pathname.startsWith('/dist/')) {
    event.respondWith((async () => {
      const cached = await caches.match(request);
      if (cached) return cached;

      const response = await fetch(request);
      if (response && response.ok && response.type === 'basic') {
        const cache = await caches.open(CACHE_NAME);
        await cache.put(request, response.clone());
      }
      return response;
    })());
    return;
  }

  // HTML navigations: network-first for freshness
  if (request.mode === 'navigate') {
    event.respondWith((async () => {
      try {
        const response = await fetch(request);
        if (response && response.ok && response.type === 'basic') {
          const cache = await caches.open(CACHE_NAME);
          await cache.put(request, response.clone());
        }
        return response;
      } catch (_err) {
        return (await caches.match(request)) || (await caches.match('/'));
      }
    })());
    return;
  }

  // Other static assets: stale-while-revalidate
  event.respondWith((async () => {
    const cached = await caches.match(request);
    const networkPromise = fetch(request)
      .then(async (response) => {
        if (response && response.ok && response.type === 'basic') {
          const cache = await caches.open(CACHE_NAME);
          await cache.put(request, response.clone());
        }
        return response;
      })
      .catch(() => null);

    if (cached) {
      void networkPromise;
      return cached;
    }

    const networkResponse = await networkPromise;
    if (networkResponse) return networkResponse;
    throw new Error('Network request failed and no cache entry available.');
  })());
});
