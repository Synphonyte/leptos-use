// Minimal service worker for `tests/use_service_worker.rs`. It lives in the crate
// root so that its scope covers the test page, which is served from `/`.
self.addEventListener("message", (event) => {
    if (event.data === "skipWaiting") {
        self.skipWaiting();
    }
});
