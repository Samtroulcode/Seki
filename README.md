# Seki go engine

> **A pure-Rust, no-std-ready Go (Weiqi/Baduk) engine designed as a reusable library for 19 × 19 (and smaller) boards.**  
> Clean API · Zero unnecessary dependencies · Fully tested · Target to be easily embeddable in CLI/TUI/GUI/Web.

| | |
|---|---|
| CI | ![CI](https://github.com/your-org/go-engine/actions/workflows/ci.yml/badge.svg) |
| Crate | <img alt="crates.io" src="https://img.shields.io/crates/v/go_engine.svg"/> |
| Docs | <img alt="docs.rs" src="https://docs.rs/go_engine/badge.svg"/> |
| License | MIT |


---

## Features

* **Idiomatic Rust API** – ergonomic types (`Coord`, `State`, `Move`) with zero-cost abstractions.  
* **Pluggable rulesets** – Japanese (default) & Chinese rules behind feature-flags.  
* **Bit-board core** – fast group detection, incremental Zobrist hashing, no `unsafe` by default.  
* **SGF I/O** – read/write SGF v4 files (optional).  
* **no-std support** – opt-in via `--no-default-features`.  
* **Criterion benches & ≥ 80 % coverage** baked into CI pipeline.

---

## Installation (to be planned)

```bash
# Engine only (library)
cargo add seki
# Engine workspace (clone with CLI & benches)
git clone https://github.com/your-org/go-engine && cd go-engine
```

---

## Quick Start (to be planned)

```rust
use seki::prelude::*;

fn main() -> Result<(), GoError> {
    // Japanese rules on a standard 19×19 board
    let mut game = State::new_japanese(BoardSize::N19);

    game.play(Coord::try_from("D4")?)?; // black stone
    game.play(Coord::try_from("Q16")?)?; // white stone

    println!("{}", game.board()); // ASCII snapshot
    Ok(())
}
```


