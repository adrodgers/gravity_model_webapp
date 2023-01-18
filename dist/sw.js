// var cacheName = 'gravity_model_webapp_pwa';
// var filesToCache = [
//   './',
//   './index.html',
//   './gravity_model_webapp.js',
//   './gravity_model_webapp_bg.wasm',
// ];

// /* Start the service worker and cache all of the app's content */
// self.addEventListener('install', function (e) {
//   e.waitUntil(
//     caches.open(cacheName).then(function (cache) {
//       return cache.addAll(filesToCache);
//     })
//   );
// });

// /* Serve cached content when offline */
// self.addEventListener('fetch', function (e) {
//   e.respondWith(
//     caches.match(e.request).then(function (response) {
//       return response || fetch(e.request);
//     })
//   );
// });
