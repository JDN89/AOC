# Day 06

## Pre Optimization

```
AOC/2024/rust on  main [✘!?] is 📦 v0.1.0 via 🦀 v1.92.0 took 19m22s
❯ hyperfine -N --warmup 5 './target/release/aoc_2024 06 p2'
Benchmark 1: ./target/release/aoc_2024 06 p2
  Time (mean ± σ):     363.1 ms ±   5.2 ms    [User: 360.1 ms, System: 2.0 ms]
  Range (min … max):   354.5 ms … 371.0 ms    10 runs
```

### Remove grid.clone()

## Optimization Ideas

Potential improvements to explore once correctness is locked in:

* Avoid `grid.clone()` inside the obstacle loop by introducing explicit reset logic.
* Replace `HashSet` usage with a fixed-size array (e.g. a 3D array indexed by row, column, and direction).
* Experiment with parallelization using **Rayon**.
* Add memoization where repeated states are recomputed.
* Use performance tools (`perf`, `hyperfine`) to measure the impact of changes.

Also worth adding:

* A Markdown linter and **Prettier** configuration for consistent formatting.

---

I’ve started structuring my data more explicitly using structs and enums. In particular, I’m relying more on enums to represent the different states and movements of the guard.

By matching on these enum states, the control flow becomes much clearer, and the overall logic is easier to reason about. The code also reads more cleanly as a result.

Working with enums and state machines still isn’t second nature to me, but I can already see how much they improve both clarity and maintainability. This is something I want to apply more consistently in my day-to-day work as well.

Part 1 of Day 6 wasn’t too difficult. Once I became more comfortable with Rust’s syntax, progress picked up. That said, I still spent a fair amount of time looking up syntax and standard library methods.

---

# Day 05

## Running `hyperfine` Correctly

Example command:

```bash
hyperfine -N --warmup 5 './target/release/aoc_2024 05 p1'
```

Output:

```text
Benchmark 1: ./target/release/aoc_2024 05 p1
  Time (mean ± σ):       3.3 ms ±   0.4 ms    [User: 1.9 ms, System: 1.2 ms]
  Range (min … max):     1.9 ms …   4.4 ms    1227 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs.
```

Anything consistently under ~10 ms generally isn’t worth optimizing further.

## `.entry()` API on HashMaps

The `.entry()` API combines:

* checking whether a key exists, and
* inserting a value if it doesn’t.

This avoids separate `contains_key` and `insert` calls and leads to cleaner code.

---

# Day 01

## Running a Specific Integration Test

Use `--test <name>` to run a specific integration test. This flag can be specified multiple times and supports common Unix glob patterns.

## `filter_map()`

`filter_map()` filters and maps at the same time: it keeps only the values for which the closure returns `Some(value)`.

Also learned a bit more about **Prettier** configuration here.

## Performance Notes

At this point, the program runs too fast to measure meaningfully—it finishes in microseconds. Once execution time increases, tools like `perf record` and `perf report` become useful.

Useful reading:

* [The Competitive Programmer’s Handbook – Rust Edition](https://book.micheletti.io/binary_search.html)

### Demangling Symbols

* [Switching to Rust’s own mangling scheme on nightly](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)

### Flamegraphs

Run with:

```bash
cargo flamegraph -- 01 p2
```

Arguments after `--` are passed to the binary.

## Debug vs Release Builds

```bash
hyperfine \
  'cargo run -- 01 p2' \
  './target/release/aoc_2024 01 p2'
```

## Showing Output During Tests

By default, Rust hides stdout during tests. To show it:

```bash
cargo test -- --nocapture
```

## `fs::read_to_string` Can Be Inefficient

I learned this around Day 5 after watching the *One Billion Row Challenge* implementation in Rust and noticing the use of `BufReader`.

References:

* [Optimizing file reading in Rust using BufReader](https://sowft.com/blog/optimizing-file-reading-in-rust-using-bufreader-for-large-files/)

`read_to_string` loads the entire file into memory at once. For large files, this can lead to high memory usage and slower performance.

### Better Approach

Using `std::io::BufReader` allows you to read the file in chunks and process it line by line, which is much more memory-efficient.

**Note:** My current approach still collects all input into memory before passing it to the AoC solution logic. A more efficient design would process lines incrementally (e.g. `reader.lines()`), but for Advent of Code problems it’s often simpler to have the full input available, especially when lookahead is required.
