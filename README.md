# Astroid Rust

A minimal Macroquad project that runs natively or in a browser canvas through WebAssembly.

## Native Run

```sh
cargo run
```

## Browser Run

Install the WASM target once:

```sh
rustup target add wasm32-unknown-unknown
```

Build the browser bundle:

```sh
./scripts/build-web.sh
```

The project includes `.cargo/config.toml` so Macroquad's JavaScript browser imports are allowed during the WASM link step.

Serve the `web` directory with reload-on-build:

```sh
./scripts/dev-web.py
```

Open <http://localhost:8000>.

While `./scripts/dev-web.py` is running, edits to Rust source files rebuild the WASM bundle and reload the browser page automatically.

Controls:

- Arrow keys change the circle velocity.
- Space resets the circle to the center.
