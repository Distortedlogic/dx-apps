---
id: dioxus-desktop
title: Dioxus Desktop
description: Desktop process behavior and current window-context access.
---

# Desktop Runtime

- Call Rust code in the same process directly. Do not add HTTP, server functions, or JavaScript IPC between local Rust components.
- Acquire the current desktop context with `dioxus::desktop::window()`. `use_window()` remains available when hook-shaped access is useful. Prefer these APIs before Tao, Wry, or direct WebView access.
