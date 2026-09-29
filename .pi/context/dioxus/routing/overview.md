---
id: dioxus-router
title: Dioxus Router
description: Reactive route parameters and current-route access.
---

# Dioxus Routing

- Route variant fields become component props. Accept route-derived values as `ReadSignal<T>` when a memo, resource, or effect must restart after navigation.
- `use_route::<Route>()` returns `Route` directly and must run below a `Router`. Use `navigator()` or `use_navigator()` for typed programmatic navigation.
