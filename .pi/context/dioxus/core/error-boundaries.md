---
id: dioxus-error-boundaries
title: Error Boundaries
description: Result propagation and captured errors from rendering, async work, and event handlers.
---

# Error boundaries

An event handler can return `Result`. Use `?` inside the handler to send a failure to the nearest `ErrorBoundary` instead of manually storing an error string.

Use `ErrorBoundary` around the smallest subtree that can recover. Its `ErrorContext` can inspect captured errors and downcast them to a domain type for a specific fallback. Rendering, async work, and handlers can bubble errors to a boundary; an unhandled error reaches the root boundary.

Do not depend on a boundary to catch Rust panics on WebAssembly, where panic catching is not available.
