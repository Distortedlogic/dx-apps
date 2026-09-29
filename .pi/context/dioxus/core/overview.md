---
id: dioxus-core
title: Dioxus Core
description: Dioxus 0.7 component, signal-prop, and Cargo-feature guardrails.
---

# Dioxus Core

- Use Dioxus 0.7.10. Never use `Scope`, `cx.render`, `use_state`, or legacy component-context APIs.
- Use `ReadSignal<T>` for read-only component and route props. `Signal<T>`, `Memo<T>`, and plain values can convert into it. Use `WriteSignal<T>` or `Signal<T>` only when the receiving component must write.
- In project Cargo manifests, set Dioxus features only from `web`, `desktop`, `mobile`, `fullstack`, `server`, and `router`. Adapt the code instead of adding any other Dioxus feature.
