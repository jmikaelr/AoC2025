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
| [Day 1](./src/bin/01.rs) | `69.4µs` | `68.1µs` |
| [Day 2](./src/bin/02.rs) | `1.4ms` | `1.6ms` |
| [Day 3](./src/bin/03.rs) | `692.5µs` | `80.1µs` |
| [Day 4](./src/bin/04.rs) | `156.3µs` | `1.6ms` |
| [Day 5](./src/bin/05.rs) | `87.5µs` | `32.8µs` |
| [Day 6](./src/bin/06.rs) | `197.4µs` | `135.4µs` |
| [Day 7](./src/bin/07.rs) | `57.6µs` | `66.2µs` |
| [Day 8](./src/bin/08.rs) | `22.7ms` | `21.7ms` |
| [Day 9](./src/bin/09.rs) | `549.3µs` | `11.4ms` |
| [Day 10](./src/bin/10.rs) | `298.4µs` | `7.1ms` |
| [Day 11](./src/bin/11.rs) | `118.2µs` | `377.7µs` |
| [Day 12](./src/bin/12.rs) | `113.8s` | `-` |

**Total: 113870.49ms**
<!--- benchmarking table --->

## Layout

- `src/bin/XX.rs` — per-day solutions.
- `data/XX.txt` — puzzle inputs.
- `src/template` — shared harness and utilities (runner, timing, README updater).
