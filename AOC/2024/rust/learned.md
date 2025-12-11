# Day 01

## Test specific integration test

--test name… Test the specified integration test. This flag may be specified
multiple times and supports common Unix glob patterns.

filter_map() filtert en map tegelijkertijd: het houdt alleen de waarden over
waarvoor de closure Some(value) teruggeeft.

Ook wat bijgeleerd over prettierrc :p

My program is to fast to time it. It finishes in microseconds. well as soon as
it start to take longer i can start trying to use perf record and report!

[The competitive programmers handbook - rust Edition](https://book.micheletti.io/binary_search.html).
Read through!

### demangling symbols

[Switching to Rust's own mangling scheme on nightly](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)

### flamegraph

call it with

```
cargo flamegraph -- 01 p2 // after -- come your arguments
```

## Compare debug vs release build

hyperfine \
 'cargo run -- 01 p2' \
 './target/release/aoc_2024 01 p2'

## show STD in out output for test

By default rust hids output from test executions. To show the output, add the
flag -- --no capture

## fs_read_to_string can be infeffecient

I learned this around day5 by watching. (Impl rust: One Billion row
challenge)[https://www.youtube.com/watch?v=tCY7p6dVAGE&t=2063s] and seeing that
Jon uses a Buffread)

[fs_read_to_string can be infeffecient](https://sowft.com/blog/optimizing-file-reading-in-rust-using-bufreader-for-large-files/)

it reads the entire file into memory using read_to_string. For large files, this
can lead to **high memory** usage and slow performance.

The Solution: Use BufReader To optimize this code, we can use
std::io::BufReader, which reads the file in chunks and allows us to process it
line by line. This approach is much more memory-efficient, especially for large
files.

**NOTE** The way I do it is not efficient anyway, because at the end I still do
.collect and pass the whole contents to the file to the AOC day{x}, where the
logic for finding the solution is located. Instead of doing somethings lik
reader.lines().execute logic. With reader.lines() we read a chunk of the file.
and then execute logic on it, instead of rading the whole file. But for AOC I
think I need the whole file anyway, otherwise I have to start looking ahead etc
