---
id: dioxus-server-integration
title: Server Integration
description: Native Dioxus server extension points and the boundary for an existing Axum host.
---

# Custom server integration

Keep backend operations as generated Dioxus endpoints. Use `#[get]`, `#[post]`, `#[put]`, or `#[delete]`, call the generated Rust function from the client, and let Dioxus register the endpoint.

Use native Dioxus extension points first:

1. Put endpoint-specific middleware on the generated function with `#[middleware(...)]`.
2. Put server-only request extractors after the route in the endpoint attribute.
3. Use `std::sync::LazyLock<T>` for synchronous process-wide services.
4. Use `dioxus::fullstack::Lazy<T>` for services that need asynchronous initialization.
5. Use Dioxus request and response types such as `Json`, `Form`, `SetHeader`, `Redirect`, `HttpError`, `ServerEvents`, and `Streaming`.

Use `dioxus::serve` with `dioxus::server::router(app)` only when the application needs router-wide middleware or required non-Dioxus routes. Add those layers and routes to the Dioxus router without rebuilding generated endpoints or changing their paths.

Use direct Axum state only when the task explicitly integrates Dioxus into an existing Axum host or a required package exposes an Axum layer. Keep `State<T>`, request extensions, `FromRef<FullstackContext>`, and host-specific Tower layers at that outer boundary.

For a custom wire type, pair the server `IntoResponse` implementation with the client `FromResponse` implementation. Pair a custom server extractor with serialization or Dioxus `IntoRequest` on the client. Keep one generated operation as the shared client and server API.
