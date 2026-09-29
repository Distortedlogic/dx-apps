---
id: dioxus-mounted-elements
title: Mounted Elements
description: Element measurement, focus, scrolling, and other renderer-backed mounted operations.
---

# Mounted elements

Receive renderer-backed element access through `onmounted`. Call `event.data()` to obtain `Rc<MountedData>`, and store it in a signal only when later events or tasks need the handle.

Use `MountedData` for operations such as `get_client_rect`, `set_focus`, and scrolling. These operations can be asynchronous because a renderer may need to finish layout or release its main thread. Handle unsupported-operation errors for renderers that do not provide the requested capability.

Prefer this API over direct DOM, WebView, or browser access.
