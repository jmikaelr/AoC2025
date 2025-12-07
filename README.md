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
| [Day 1](./src/bin/01.rs) | `63.2µs` | `59.4µs` |
| [Day 2](./src/bin/02.rs) | `1.3ms` | `1.5ms` |
| [Day 3](./src/bin/03.rs) | `643.9µs` | `68.4µs` |
| [Day 4](./src/bin/04.rs) | `146.7µs` | `1.5ms` |
| [Day 5](./src/bin/05.rs) | `82.9µs` | `30.9µs` |
| [Day 6](./src/bin/06.rs) | `203.3µs` | `126.4µs` |
| [Day 7](./src/bin/07.rs) | `54.6µs` | `62.8µs` |

**Total: 5.84ms**
<!--- benchmarking table --->

## Layout

- `src/bin/XX.rs` — per-day solutions.
- `data/XX.txt` — puzzle inputs.
- `src/template` — shared harness and utilities (runner, timing, README updater).
