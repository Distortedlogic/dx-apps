---
id: dioxus-fullstack-hydration
title: Hydration Values
description: Choosing use_loader or use_server_cached for server-to-client hydration data.
---

# Hydration values

Use `use_loader` for asynchronous render-required data. The server waits for the first result, serializes it, and the hydrating web client reuses it.

Use `use_server_cached` only for synchronous serializable values that must match between server rendering and hydration, such as a generated element ID or initial non-secret configuration. Its closure runs on the server and the result is read in the same hook position on the client. Keep initial hook order identical on both sides.

A value sent through either mechanism reaches the client. Keep credentials, private keys, and server-only configuration out of hydration data.

Use ordinary `use_hook` for desktop, mobile, or client-only initialization that does not cross an SSR boundary.
