# Advent of Code 2025 — Rust

Another December, another pile of puzzles. I’m using the solid [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) template and trying to keep the solutions tidy and quick.

## Running things

- Single day: `cargo run --release --bin 03`
- Everything solved so far: `cargo run --release --all`
- Tests: `cargo test --all`
- Benchmarks (also updates the table below): `cargo time --all --store`

## Benchmarks

Current timings from `cargo time --all --store`:

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `64.0µs` | `59.6µs` |
| [Day 2](./src/bin/02.rs) | `8.2ms` | `5.6ms` |
| [Day 3](./src/bin/03.rs) | `632.3µs` | `71.2µs` |
| [Day 4](./src/bin/04.rs) | `143.5µs` | `1.5ms` |
| [Day 5](./src/bin/05.rs) | `82.5µs` | `30.4µs` |
| [Day 6](./src/bin/06.rs) | `189.4µs` | `127.2µs` |
| [Day 7](./src/bin/07.rs) | `59.8µs` | `63.5µs` |

**Total: 16.82ms**
<!--- benchmarking table --->

## Layout

- `src/bin/XX.rs` — per-day solutions.
- `data/XX.txt` — puzzle inputs.
- `src/template` — shared harness and utilities (runner, timing, README updater).
