---
id: dioxus-project-setup
title: Project Setup
description: Cargo features, current Dioxus.toml keys, launch configuration, and target commands.
---

# Project setup

Use this reference only to create a Dioxus target or change project configuration.

## Cargo features

Map project features to the required Dioxus renderers:

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"]
```

Add `dioxus/fullstack` to the project feature that enables generated client-server operations. Make server-only dependencies optional, and put their `dep:` entries in the server feature.

Use one shared `dioxus::launch(App)` entry point when default launch behavior is sufficient. Use `LaunchBuilder::desktop().with_cfg(...)` only for initial desktop window configuration. In a multi-renderer entry point, use `LaunchBuilder::new()` with `desktop!` and `native!` configuration blocks.

Add the router dependency or feature only when the application has routes. Do not add another web bundler or development server.

## Dioxus.toml

Add `Dioxus.toml` only for CLI, application, bundle, or platform configuration. Current 0.7 keys include `[application].default_platform`, `out_dir`, and `asset_dir`, plus `[web.app].title`. Put shared identity in `[bundle]`, iOS deployment and background settings in `[ios]`, Android SDK and hardware settings in `[android]`, and supported cross-platform permissions in `[permissions]`.

## Target commands

Use the command for the selected target:

```sh
dx serve --platform web
dx serve --platform desktop
dx serve --platform ios
dx serve --platform android
```

A web full-stack client normally uses the current origin. Set `dioxus::fullstack::set_server_url` for a desktop or mobile client only when deployment uses a remote server.
