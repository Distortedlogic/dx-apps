---
id: dioxus-state-store
title: State Store
description: Lens-first nested state, transposition, collections, component boundaries, and domain operations.
---

# Store state

Use `Store` for a large nested domain model that needs field-level or item-level reactive reads and writes. Keep small independent component state in `use_signal`.

```rust
#[derive(Store, Clone, PartialEq)]
struct AppState {
    items: Vec<Item>,
    selected: Option<u32>,
}

let state = use_store(|| AppState {
    items: Vec::new(),
    selected: None,
});
```

## Preserve the Store graph

Keep `Store` handles and generated projections through components and event handlers. A projection such as `state.items()` is a Store scoped to that field and shares the parent Store subscription graph.

A read of the root or a parent projection subscribes to changes below that point. Preserve granularity by projecting to the required field or item before reading it. Keep stored model fields as plain data and let Store provide their reactivity.

Avoid reading or cloning the root value, destructuring that snapshot, and placing its fields in new signals. That creates broad subscriptions and disconnected state. Take a complete snapshot only for an operation that needs the complete value, such as persistence or export.

Read and mutate the narrowest projection:

```rust
let mut items = state.items();
let mut selected = state.selected();

items.push(new_item);
selected.set(Some(id));
```

Use `state.set(new_state)` or a root write only for an intentional complete-model replacement, such as a reset or snapshot load.

## Struct and enum projections

`#[derive(Store)]` generates one projection method for each struct field. `transpose()` returns the generated `<Type>StoreTransposed` value whose fields are Store handles, not copied values:

```rust
let AppStateStoreTransposed { items, selected } = state.transpose();
```

Use this form when destructuring is clearer than repeated projection calls. Continue to pass and use the projected stores directly.

For a derived enum, use generated `is_<variant>()` methods for a shallow variant check. A one-field variant also gets a method that returns `Option<Store<Field, _>>`. Use the generated `<Enum>StoreTransposed` enum to match a variant while retaining Store handles for its fields.

## Optional and fallible state

For `Store<Option<T>>`, use `transpose()` to get `Option<Store<T, _>>`. It tracks the `None` or `Some` transition without subscribing to every inner field.

```rust
match state.current_user().transpose() {
    Some(user) => rsx! { UserView { user } },
    None => rsx! { LoginPrompt {} },
}
```

Use `is_some`, `is_none`, `is_some_and`, `filter`, `as_deref`, `unwrap`, or `expect` on the Store when their normal `Option` semantics fit the task. Prefer `transpose` when the UI must continue to work with the inner Store.

For `Store<Result<T, E>>`, use `transpose()` to get `Result<Store<T, _>, Store<E, _>>`. The `ok`, `err`, `is_ok`, `is_err`, `unwrap`, `expect`, `unwrap_err`, `expect_err`, and `as_deref` methods also preserve Store projections.

Use `store.deref()` for a stored `Box` or another `DerefMut` wrapper when the UI needs a Store projected to its target.

## Collections

Use Store collection methods instead of writing the complete collection.

- `Vec`: `len` and `is_empty` track the shape. `iter` and `get` return stores scoped to items. `push` dirties only the length. `insert`, `remove`, and `retain` dirty the affected index range and length. `clear` invalidates the collection.
- `HashMap` and `BTreeMap`: `len`, `is_empty`, `contains_key`, `iter`, and `values` track key-set changes. `get` returns a store scoped to one key. Use `insert`, `remove`, `retain`, and `clear` for granular mutations.

Use `get_unchecked` only when the key is known to exist; reading a missing key panics. Prefer a map keyed by stable IDs when item identity must survive insertion, removal, or reordering. A `Vec` projection is index-based, so edits can invalidate items at and after the changed index.

Pass item stores to row components. Do not clone each item into a signal.

## Component boundaries

Pass `Store<T>` or a projected Store when a child needs generated field or collection APIs. Use `ReadStore<T>` or `WriteStore<T>` when a component boundary must erase the concrete lens and restrict its capability.

A Store can convert to `ReadSignal` or `WriteSignal` for an API that requires a signal. Convert the narrow projected Store, not the root Store followed by value destructuring. Render or format a projected Store directly when its value supports it, and call `.cloned()` only when code needs an owned snapshot.

Stores are copyable when their lens is copyable. Move Store handles into closures without cloning their values.

## Application models and providers

For a UI that coordinates a Store with async commands, create one copyable model that contains the domain `Store` and its `Action` handles. Initialize it in one custom hook, provide the model through context, and let child components request that model instead of recreating command state.

Expose semantic projection methods from the model or a `#[store]` extension. A projection method can chain generated lenses and return `Store<Field, impl Writable<Target = Field> + Copy + 'static>`. Keep data access granular even when the path crosses several nested structs.

Provide separate Stores by domain type when their lifecycles are independent. Prefer subtree context providers over `GlobalStore`; reserve a global Store for state that truly spans every application root.

## Complex forms

Use one nested Store for a large or multi-step form. Derive `Store` on nested form sections and pass projected section Stores to each step component. Keep wizard navigation, modal state, and other temporary UI controls separate from submitted form data.

Read the complete form value once at the submission boundary, convert it to a shared request DTO, and submit it with `use_action`. Use the action's pending and error state to disable duplicate submission and show feedback. Continue editing through field projections after a failed submission.

## Domain operations

Put model operations on Store extensions:

```rust
#[store]
impl<Lens> Store<AppState, Lens> {
    fn select(&mut self, id: u32) {
        self.selected().set(Some(id));
    }
}
```

A method taking `&self` receives a generated `Readable` lens bound. A method taking `&mut self` receives a `Writable` lens bound. Use `#[store(pub)]` or a named extension trait only when the generated trait must cross a module boundary.

Keep component event code as a caller of these operations. Keep low-level `SelectorScope` mapping for reusable collection or library abstractions; normal application models should use derived projections.

## Storage choice

Use `use_store` for normal component-local Store state. Use `use_store_sync` only when the complete stored type must satisfy `Send + Sync`. Use `GlobalStore<T>` only for state that must be globally resolved rather than provided through component context.
