---
id: dioxus-eval-bridge
title: Eval Widget Bridge
description: Lifecycle-safe typed messaging for a required external JavaScript widget.
---

# Eval widget bridge

Use `document::eval` only when a required external JavaScript widget has no suitable Dioxus or maintained Rust integration.

Initialize an Eval handle synchronously with `use_hook` when child tasks or fast producers can send during the first render. Keep a populated handle, such as `Signal<document::Eval>`, instead of initializing `Option<Eval>` later in `use_effect`; that later initialization can drop the first messages.

Give each widget a stable element ID. In a full-stack web build, use `use_server_cached` for a generated ID that must match between server rendering and hydration.

Send small typed JSON commands with `Eval::send` and decode typed replies with `Eval::recv`. Use `peek` when sending must not create a reactive dependency. For binary data, define one explicit transport encoding and bound payload size.

Send the widget's dispose command from `use_drop`. Keep widget creation, message schema, resize, and disposal behind one typed Rust handle.
