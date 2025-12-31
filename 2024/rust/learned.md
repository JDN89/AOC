# Day 06

Ik ben mijn data meer beginnen opdelen in structs en enums. Ook ben ik vaker enums gaan gebruiken om de verschillende bewegingen die een guard kan maken op te splitsen. Door te matchen op deze enum-states kan ik de logica duidelijker structureren, en zo werd de code ook veel overzichtelijker.

Ik ben dit nog niet echt gewoon — werken met enums en state machines — maar ik merk dat het de logica én de leesbaarheid sterk verbetert. Daarom wil ik dit ook meer beginnen toepassen op mijn werk.

Deel 1 van dag 6 was niet zo moeilijk. Eens je de rust syntax wat gewoon bent gaat het iet wat vooruit, maar ben er toch redelijk lang nog mee bezig geweest, omdat ik nog vaal de syntax en de verschillende methodes moet raadplegen.

# Day 05

## running hyperfine correclty

hyperfine -N --warmup 5 './target/release/aoc_2024 05 p1'

```markdown
rust on  main is 📦 v0.1.0 via 🦀 v1.92.0
❯ hyperfine -N --warmup 5 './target/release/aoc_2024 05 p1'

Benchmark 1: ./target/release/aoc_2024 05 p1
  Time (mean ± σ):       3.3 ms ±   0.4 ms    [User: 1.9 ms, System: 1.2 ms]
  Range (min … max):     1.9 ms …   4.4 ms    1227 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```

Alles onder 10ms is niet de moeite om te optimaliseren denk ik.

## .entry api on hasmaps

Combines checking of iets bestaat in de map en inserten van values (if exists), zoniet insert default value


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
