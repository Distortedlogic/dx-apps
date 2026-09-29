---
id: dioxus-advanced-routing
title: Advanced Routing
description: Typed query and hash syntax, redirects, route order, and scroll restoration.
---

# Advanced routing

Keep URL state in the typed `Routable` enum instead of parsing location strings.

- Use `#[route("/search?:query&:word_count")]` for typed query fields.
- Use `#[route("/?:..query")]` when one type owns the complete query string.
- Use `#[route("/#:state")]` for typed hash state.
- Use typed redirects for old or canonical paths.
- Use router scroll restoration instead of a global browser scroll listener.

Keep layout and nest order explicit. Put a catch-all route after the specific routes that it must not replace.
