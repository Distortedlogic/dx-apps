---
id: dioxus-mobile
title: Dioxus Mobile
description: Mobile target commands and generated platform permissions.
---

# Mobile Runtime

- Run mobile targets with `dx serve --platform ios` or `dx serve --platform android`.
- Declare supported capabilities in `[permissions]` in `Dioxus.toml`; let the Dioxus CLI generate Android and iOS metadata. Put platform-only additions under `[android.permissions]` or `[ios.plist]`.
