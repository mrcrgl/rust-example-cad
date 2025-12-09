# Rust CAD Mini-Workspace

An educational Rust workspace that explores fundamental language concepts and gradually introduces asynchronous programming with Tokio and shared, mutable state. The project is intentionally small and focused so you can learn by reading and extending real code.

## Learning goals

- Understand basic Rust project structure with a Cargo workspace
- Work with modules, visibility, traits, and dynamic dispatch (`Box<dyn Trait>`)
- Use external crates (`clap` for CLI, `rand` for randomness)
- Implement domain types, traits, and separation of concerns across crates
- Prepare for asynchronous programming with Tokio
- Safely share mutable state across tasks with `Arc<Mutex<T>>` and `Arc<RwLock<T>>`

## Project structure

Workspace layout:
~~~
rust-202512-2/
├─ Cargo.toml          # Workspace definition
├─ cad-geometry/       # Library crate: core geometry domain
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     ├─ area.rs       # `ToArea` trait
│     ├─ figures/      # Shapes + traits
│     │  ├─ mod.rs
│     │  ├─ figure.rs  # `Figure` trait + `FigureType`
│     │  ├─ circle.rs
│     │  └─ rectangle.rs
│     └─ application/
│        ├─ mod.rs
│        └─ figure_producer.rs  # Produces random figures (currently circles)
└─ cad-cli/            # Binary crate: command-line app
   ├─ Cargo.toml
   └─ src/
      ├─ main.rs       # Entry point
      ├─ cli.rs        # `clap`-powered CLI definitions
      ├─ command.rs    # Command routing
      └─ command/
         └─ produce_areas.rs
~~~

### Crate: `cad-geometry` (library)
- `area::ToArea`: A trait to compute the area for geometric shapes.
- `figures::Figure` and `figures::FigureType`: A trait and enum that categorize and display figure types. Shapes implement `Figure` and `ToArea`.
- Shapes:
  - `figures::circle::Circle`
  - `figures::rectangle::Rectangle`
- `application::figure_producer::GeometricFigureProducer`: Produces a `Vec<Box<dyn Figure>>` using the `rand` crate. Currently generates circles with random radii to demonstrate dynamic dispatch and trait objects.

Key Rust concepts:
- Traits and trait bounds
- Dynamic dispatch via `Box<dyn Figure>`
- Module organization and re-exports (`pub use`)
- Crate boundaries and reuse across a workspace

### Crate: `cad-cli` (binary)
- Uses `clap` to define a small CLI.
- Subcommand: `produce-areas --amount <N>`
- Workflow:
  1. Parse CLI args via `clap`.
  2. Call into the `cad-geometry` library (`GeometricFigureProducer`) to produce figures.
  3. Print each figure’s type and area.

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
# Produce 5 random figures and print their areas
cargo run -p cad-cli -- produce-areas --amount 5
~~~

Expected output (example):
~~~
todo produce 5 areas
circle(31415.927)
circle(7853.981)
circle(1.256)
...
~~~

Note: The first line is a placeholder message printed by the current implementation; you can safely remove or update it as you evolve the command.

## What’s covered so far

- Cargo workspace with a library crate (`cad-geometry`) and a binary crate (`cad-cli`)
- CLI parsing with `clap`
- Traits (`ToArea`, `Figure`), trait objects (`Box<dyn Figure>`), and dynamic dispatch
- Shape implementations (`Circle`, `Rectangle`)
- Simple “application service” that returns abstracted domain objects
- Random value generation with `rand`

## Roadmap: introducing Tokio and shared mutable state

We will extend the CLI and library to demonstrate async programming with Tokio and safe shared state patterns.

Planned additions:
1. Add `tokio` as a dependency to `cad-cli`.
2. Convert `main` into an async entrypoint (e.g., `#[tokio::main] async fn main() { ... }`).
3. Introduce concurrent tasks that:
   - Generate figures concurrently
   - Compute areas concurrently
4. Aggregate results from tasks using shared mutable state guarded by:
   - `Arc<Mutex<Vec<f32>>>` (exclusive lock) or
   - `Arc<RwLock<Vec<f32>>>` (read/write lock)
5. Add a new CLI subcommand (e.g., `produce-areas-async`) to compare synchronous vs asynchronous approaches.

Sketch of upcoming patterns:

- Async entrypoint and task spawning:
~~~rust
#[tokio::main]
async fn main() {
    // parse CLI...
    // tokio::spawn tasks that produce/compute areas
    // await joins, aggregate results
}
~~~

- Shared aggregation with `Arc<Mutex<T>>`:
~~~rust
use std::sync::{Arc, Mutex};
let areas = Arc::new(Mutex::new(Vec::new()));
// move `areas.clone()` into tasks; lock, push results
~~~

- For read-mostly scenarios, prefer `Arc<RwLock<T>>` to allow concurrent reads.

- Consider channels (`tokio::sync::mpsc`) to decouple producers and consumers without sharing a collection directly.

Discussion points we’ll cover:
- When async helps (I/O-bound, high-concurrency tasks) vs when it’s unnecessary (pure CPU-bound workloads without I/O)
- Minimizing lock contention and avoiding deadlocks
- Choosing between channels and shared state
- Error handling in async contexts (`anyhow`, `thiserror`), and graceful task shutdown

## Suggested exercises

- Library:
  - Implement `ToArea` for `Rectangle` and update the producer to generate both circles and rectangles.
  - Add a new shape (e.g., `Triangle`) and wire it through `FigureType`.
  - Implement `Display` for shapes to print dimensions nicely.

- CLI:
  - Add a `--shape` filter or ratio flags (e.g., `--circle-weight 70 --rectangle-weight 30`).
  - Add `--min-radius` / `--max-radius` bounds for circles, with validation.

- Async and shared state:
  - Add `produce-areas-async` subcommand using Tokio tasks.
  - Aggregate areas with `Arc<Mutex<Vec<f32>>>` and print summary stats (count, min, max, average).
  - Replace the mutex with a channel-based pipeline and compare performance/complexity.

- Testing and quality:
  - Add unit tests for `to_area` implementations.
  - Add property tests (e.g., area is non-negative).
  - Run `cargo fmt` and `cargo clippy` in CI or locally.

## Tips

- Prefer traits to decouple generation and consumption of domain objects.
- Keep the CLI thin; put logic into the library to make it testable and reusable.
- When introducing async, isolate blocking work (e.g., heavy CPU tasks) to avoid stalling the executor, or use `spawn_blocking` if appropriate.
- Start simple: get correctness first, then iterate toward concurrency and shared state.

## License

This project is intended for educational use. Use, modify, and extend it as you learn.