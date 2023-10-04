var buildVersion = "{{buildVersion}}"
var cssBuildVersion = "{{cssBuildVersion}}"
var cacheName = "demo";
var filesToCache = [
    './',
    './index.html',
    './manifest.json',
    './use_service_worker-' + buildVersion + '_bg.wasm',
    './use_service_worker-' + buildVersion + '.js',
    './output-' + cssBuildVersion + '.css',
    './res/icon/maskable_icon_x48.png',
    './res/icon/maskable_icon_x72.png',
    './res/icon/maskable_icon_x96.png',
    './res/icon/maskable_icon_x128.png',
    './res/icon/maskable_icon_x192.png',
    './res/icon/maskable_icon_x384.png',
    './res/icon/maskable_icon_x512.png',

    // TODO: Add files you want the SW to cache. Rename entries to match your build output!
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (event) {
    console.log("Installing service-worker for build", buildVersion);
    const preCache = async () => {
        get_cache().then(function (cache) {
            // We clear the whole cache, as we do not know which resources were updated!
            cache.keys().then(function (requests) {
                for (let request of requests) {
                    cache.delete(request);
                }
            });
            cache.addAll(filesToCache.map(url => new Request(url, { credentials: 'same-origin' })));
        })
    };
    event.waitUntil(preCache);
});

self.addEventListener('message', function (messageEvent) {
    if (messageEvent.data === "skipWaiting") {
        console.log("Service-worker received skipWaiting event", buildVersion);
        self.skipWaiting();
    }
});

self.addEventListener('fetch', function (e) {
    e.respondWith(cache_then_network(e.request));
});

async function get_cache() {
    return caches.open(cacheName);
}

async function cache_then_network(request) {
    const cache = await get_cache();
    return cache.match(request).then(
        (cache_response) => {
            if (!cache_response) {
                return fetch_from_network(request, cache);
            } else {
                return cache_response;
            }
        },
        (reason) => {
            return fetch_from_network(request, cache);
        }
    );
}

function fetch_from_network(request, cache) {
    return fetch(request).then(
        (net_response) => {
            return net_response;
        },
        (reason) => {
            console.error("Network fetch rejected. Falling back to ./index.html. Reason: ", reason);
            return cache.match("./index.html").then(function (cache_root_response) {
                return cache_root_response;
            });
        }
    )
}
