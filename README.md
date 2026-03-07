# SamTV Homepage (Leptos CSR)

This project has been migrated from Astro to a Rust + Leptos client-side rendered app.

## Stack

- Rust
- Leptos (`csr`)
- Leptos Router
- Trunk (bundling + wasm build output to `dist/`)
- Nginx container serving `dist/`

## Run locally

1. Install Trunk if needed:

```bash
cargo install trunk
```

2. Start the dev server:

```bash
trunk serve --port 3000 --open
```

## Production build

```bash
trunk build --cargo-profile release
```

Build output is written to `dist/`, so the existing `Dockerfile` still works unchanged.
