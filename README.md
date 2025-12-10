# Rust CAD Mini-Workspace

An educational Rust workspace that explores fundamental language concepts and gradually introduces asynchronous programming with Tokio, shared mutable state, and a small REST API powered by Axum. The project is intentionally small and focused so you can learn by reading and extending real code.

## Learning goals

- Understand basic Rust project structure with a Cargo workspace (multi-crate)
- Work with modules, visibility, traits, and dynamic dispatch (`Arc<dyn Trait>`)
- Use external crates (`clap` for CLI, `rand` for randomness, `tokio` for async, `axum` for HTTP, `serde`/`serde_json` for JSON, `tracing` for diagnostics)
- Implement domain types, traits, and separation of concerns across crates
- Write asynchronous Rust with Tokio and spawn tasks safely
- Safely share mutable state across tasks with `Arc<Mutex<T>>` and `Arc<RwLock<T>>`
- Build a small REST API server and return typed JSON responses

## Project structure

Workspace layout:
~~~
rust-202512-2/
├─ Cargo.toml                 # Workspace definition + shared dependencies
├─ cad-geometry/              # Library crate: core geometry domain
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     ├─ area.rs              # `ToArea` trait
│     ├─ figures/             # Shapes + traits
│     │  ├─ mod.rs
│     │  ├─ figure.rs         # `Figure` trait (+Send+Sync) + `FigureType`
│     │  ├─ circle.rs
│     │  └─ rectangle.rs
│     └─ application/
│        ├─ mod.rs
│        └─ figure_producer.rs # Produces random figures (sync + async)
├─ cad-cli/                   # Binary crate: command-line app
│  ├─ Cargo.toml
│  └─ src/
│     ├─ main.rs              # Async entrypoint (`#[tokio::main]`)
│     ├─ cli.rs               # `clap`-powered CLI definitions
│     ├─ command.rs           # Command routing + timing
│     ├─ error.rs             # CLI error types + exit codes
│     └─ command/
│        ├─ produce_areas.rs  # sync + async area production
│        └─ produce_circles.rs# circle generator w/ optional JSON output
└─ cad-rest/                  # Binary crate: Axum REST server
   ├─ Cargo.toml
   └─ src/
      ├─ main.rs              # Server bootstrap, tracing, app state
      ├─ services.rs          # Routes/handlers (`/api/get_circle`)
      └─ error.rs             # Error -> HTTP response mapping
~~~

Workspace-level dependencies (example):
- `tokio`, `serde`, `serde_json`, and `tracing` are declared at the workspace level and used from member crates to keep versions consistent.

### Crate: `cad-geometry` (library)
- `area::ToArea`: A trait to compute the area for geometric shapes.
- `figures::Figure` and `figures::FigureType`: A trait and enum that categorize and display figure types. Shapes implement `Figure` and `ToArea`. The `Figure` trait is `Send + Sync` to support concurrency.
- Shapes:
  - `figures::circle::Circle`
  - `figures::rectangle::Rectangle`
- `application::figure_producer::GeometricFigureProducer`:
  - `produce(amount) -> Vec<Arc<dyn Figure>>` returns trait objects behind `Arc` to enable sharing across async tasks.
  - `produce_async(amount) -> Vec<Arc<dyn Figure>>` builds items concurrently and awaits them via `futures::future::join_all`.
  - `produce_circles(amount) -> Vec<Circle>` returns concrete `Circle` values (used by the REST endpoint).

Key Rust concepts:
- Traits and trait bounds
- Dynamic dispatch via `Arc<dyn Figure>`
- Module organization and re-exports (`pub use`)
- Crate boundaries and reuse across a workspace
- Async fan-out/fan-in using `join_all`

### Crate: `cad-cli` (binary)
- Async CLI using `#[tokio::main]`.
- Subcommands:
  - `produce-areas --amount <N>`: synchronous generation and printing of figure areas.
  - `produce-areas-async --amount <N>`: asynchronous generation using the async producer.
  - `produce-circles --amount <N> [--json]`: generate `N` circles; print radii or pretty JSON with `--json`.
- Workflow:
  1. Parse CLI args via `clap`.
  2. Call into `cad-geometry` (`GeometricFigureProducer`) to produce figures or circles.
  3. Print each figure’s type and area (or serialize to JSON).
  4. Print elapsed time (simple performance feedback).

### Crate: `cad-rest` (binary, Axum server)
- Async HTTP server using `axum` + `tokio`, with `tracing` diagnostics.
- App state:
  - `AppState { producer: Arc<GeometricFigureProducer>, is_online: bool }`
- Routes:
  - `GET /api/get_circle?amount=N` → JSON `[Circle, ...]`
    - Default `amount=1`
    - Validates `amount <= 1000`, otherwise returns `400` with a JSON error payload
- Error handling:
  - Custom `ApiError` implements `IntoResponse`, mapping domain errors to HTTP responses and JSON bodies.
- Run configuration:
  - Binds to `127.0.0.1:1337`
  - `RUST_LOG`/`tracing_subscriber` env filter supported (e.g., `RUST_LOG=info`)

## Getting started

Prerequisites:
- A recent Rust toolchain (installed with `rustup`). This workspace uses `edition = "2024"` in `Cargo.toml`. If your toolchain does not yet support it, you can:
  - Update Rust to the latest stable toolchain, or
  - Temporarily change `edition = "2024"` to `edition = "2021"` in each crate’s `Cargo.toml`.

Build the entire workspace:
~~~sh
cargo build
~~~

Run the CLI:
~~~sh
# Produce 5 random figures and print their areas (sync)
cargo run -p cad-cli -- produce-areas --amount 5

# Produce 5 random figures using the async producer
cargo run -p cad-cli -- produce-areas-async --amount 5

# Produce 3 circles, pretty-printed JSON
cargo run -p cad-cli -- produce-circles --amount 3 --json
~~~

Run the REST server:
~~~sh
# Start the server (listens on 127.0.0.1:1337)
RUST_LOG=info cargo run -p cad-rest
~~~

Call the endpoint:
~~~sh
# Get one circle
curl -s http://127.0.0.1:1337/api/get_circle | jq .

# Get 5 circles
curl -s "http://127.0.0.1:1337/api/get_circle?amount=5" | jq .
~~~

## What’s covered so far

- Cargo workspace with a library crate (`cad-geometry`) and two binary crates (`cad-cli`, `cad-rest`)
- Workspace-level dependency management (`tokio`, `serde`, `serde_json`, `tracing`)
- CLI parsing with `clap` and an async `main`
- Traits (`ToArea`, `Figure`), trait objects behind `Arc<dyn Figure>`, and dynamic dispatch
- Shape implementations (`Circle`, `Rectangle`)
- Random value generation with `rand`
- Async figure production via `join_all`
- Axum REST server with:
  - Typed state (`AppState`) and shared components (`Arc<...>`)
  - Query parameter extraction and validation
  - JSON responses with `serde`
  - Error handling via `thiserror` + `IntoResponse`
  - Structured logging with `tracing` + `tracing-subscriber`

## Roadmap: shared mutable state and async patterns

We will extend the CLI and REST server to demonstrate additional async and shared-state patterns.

Planned additions:
1. Compare shared-state aggregation patterns:
   - `Arc<Mutex<Vec<f32>>>` vs `Arc<RwLock<Vec<f32>>>` for concurrent writers/readers
   - Channel-based pipelines (`tokio::sync::mpsc`) to reduce lock contention
2. Add metrics/state to the server:
   - Track request counts, last N generated radii, min/max/avg, etc.
   - Expose `/api/stats` that reads shared state under a read lock
3. Explore graceful shutdown and task management:
   - Use `tokio::select!` with shutdown signals
   - Structured errors (`anyhow`/`thiserror`) in async contexts
4. Add tests:
   - Unit tests for shape areas
   - Integration tests for the REST API (spawn server on a random port)

Sketches:

- Shared aggregation with `Arc<Mutex<T>>`:
~~~rust
use std::sync::{Arc, Mutex};
let areas = Arc::new(Mutex::new(Vec::new()));
// move `areas.clone()` into tasks; lock, push results
~~~

- Async entrypoint and task spawning:
~~~rust
#[tokio::main]
async fn main() {
    // parse CLI...
    // tokio::spawn tasks that produce/compute areas
    // await joins, aggregate results
}
~~~

## Suggested exercises

- Library:
  - Add `Triangle` and wire through `FigureType`.
  - Implement `Display` for shapes to print dimensions nicely.

- CLI:
  - Add `--shape` filters or ratios (e.g., `--circle-weight 70 --rectangle-weight 30`).
  - Add bounds for random generation (e.g., `--min-radius` / `--max-radius`) with validation.

- Async and shared state:
  - Extend `produce-areas-async` to aggregate stats with `Arc<Mutex<_>>` and print count/min/max/avg.
  - Replace the mutex with a channel-based pipeline and compare performance/complexity.

- REST:
  - Add `/api/get_rectangle` and `/api/stats`.
  - Validate params and return structured `ApiError`s.
  - Add tracing spans/fields per request.

- Testing and quality:
  - Add unit tests for `to_area` implementations.
  - Add property tests (e.g., area is non-negative).
  - Run `cargo fmt` and `cargo clippy` in CI or locally.

## Tips

- Prefer traits to decouple generation and consumption of domain objects.
- Keep the CLI thin; put logic into the library to make it testable and reusable.
- When introducing async, isolate blocking work (e.g., heavy CPU tasks) to avoid stalling the executor, or use `spawn_blocking` if appropriate.
- Use `Arc<Mutex<_>>` for simple shared writes; consider channels to avoid holding locks during work.
- Start simple: get correctness first, then iterate toward concurrency and shared state.

## License

This project is intended for educational use. Use, modify, and extend it as you learn.