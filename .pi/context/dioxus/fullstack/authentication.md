---
id: dioxus-fullstack-auth
title: Fullstack Authentication
description: Authentication in full-stack Dioxus applications.
---

# Full-stack authentication

Use `Form<T>` for a typed form request. Use `FormEvent::parsed_values<T>()` when a Dioxus form event must become that request. A login server function can return `SetHeader<SetCookie>`.

Use this verb-macro syntax for a typed cookie extractor:

```rust
#[get("/api/account", cookie: TypedHeader<Cookie>)]
```

Use `TypedHeader<Cookie>` for a typed cookie extractor. Keep login, logout, identity, refresh, and permission operations as stable named Dioxus endpoints.

Keep a distinct authentication-initialization state in the client model. A protected route layout must wait for initialization before it redirects or renders its `Outlet`. Use typed `navigator.replace(...)` for the login redirect, show a loading state during initialization, and render no protected content while redirecting.

Prefer HTTP-only cookie sessions for a web client. When a deployed desktop or mobile client must call a remote server and cookies do not fit, set the server URL once at startup, send a typed token request to stable named endpoints, and keep the token in secure platform storage.

When the selected authentication package needs a server layer, attach only that required layer at the existing `dioxus::server::router` boundary. Its typed session can then be a server-only extractor in a generated endpoint.
