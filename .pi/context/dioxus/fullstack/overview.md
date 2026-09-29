---
id: dioxus-fullstack
title: Dioxus Fullstack
description: Endpoint macros, wire types, loaders, actions, extractors, and error handling.
---

# Full-Stack Runtime

- Prefer `#[get]`, `#[post]`, `#[put]`, and `#[delete]` with literal routes. Use `#[server]` only for anonymous or compatibility use.
- Call generated functions directly from client Rust. Use `reqwest` only for external services.
- Keep request, response, and error types shared. Client arguments must deserialize or implement `IntoRequest`; results must deserialize or implement `FromResponse`. Put implementations and server-only extractors behind `server`; place extractors after the route.
- Use `use_loader(...)?` for new render-required data and `use_action` for explicit mutations. `use_server_future` remains supported, but prefer `use_loader` for new code.
- The first loader run can suspend rendering. Server rendering waits for it, and hydration reuses its serialized result. After the first successful load, dependency changes reload in the background without suspending again; a reload failure produces `Loading::Failed`.
- Put server-only extractors in the endpoint attribute. For low-level extraction, use `FullstackContext::extract`; the free `extract()` helper is deprecated.
- Dioxus 0.7.10 `ServerFnError` is not generic. Use Dioxus `Result<T>` normally, `HttpError` for explicit HTTP failures, and `AsStatusCode` for serializable domain errors that retain their variant.
