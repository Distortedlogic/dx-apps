---
id: dioxus-async-actions
title: Async Actions and Dependencies
description: Event-driven async actions and explicit dependencies for ordinary values.
---

# Async actions and dependencies

Use `use_action` for async work that starts from a user event. Call it with `.call(...)`, inspect the reactive `.pending()` state, and use `.cancel()` or `.reset()` when required. `.value()` returns `Option<Result<ReadSignal<T>, CapturedError>>`. Await the future returned by `.call(...)` when later code must run after that invocation settles. A new call cancels the prior in-flight call and retains only the newest result.

Use `use_resource` for async work that starts automatically and restarts when its reactive dependencies change. Use `use_future` for non-reactive background work that starts with the component. Use `spawn` only when no action or resource state is needed.

Signals read by `use_resource`, `use_memo`, and `use_effect` become dependencies. Wrap a closure with `use_reactive!` when changes to ordinary props or other non-signal values must restart that work.

Use action cancellation for debounce behavior: begin the action with the quiet delay, then read the latest Store or signal value after the delay. A newer call cancels the older delayed action.

For a long-lived service or actor, create its handle once with `use_hook`, start the actor once, and own its subscription loop with `use_future`. Apply received snapshots or events through narrow Store operations. Component disposal then owns cancellation of the subscription task.

Use `use_signal_sync` only when the value must use `SyncStorage` and satisfy `Send + Sync`. Keep component-local work on `use_signal`.
