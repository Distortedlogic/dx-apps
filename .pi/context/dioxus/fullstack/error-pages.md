---
id: dioxus-fullstack-error-pages
title: Fullstack Error Pages
description: Typed SSR error pages and HTTP status propagation through routed layouts.
---

# Full-stack error pages

Wrap routed content in an `ErrorBoundary` inside a route layout when the server-rendered page needs a typed fallback.

Use `HttpError` for failures that carry an HTTP status. In the boundary, call `FullstackContext::commit_error_status` with the captured error to preserve its status in the SSR response. Use `FullstackContext::commit_http_status` when the page must set an explicit status.

Commit the status while the initial response is still rendering. Errors that no boundary handles use the generic SSR error page.
