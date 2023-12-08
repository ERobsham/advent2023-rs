### Advent of Code 2023 - in Rust!

Playing around and learning some Rust features a bit better.

As of right now the plan is to make this act like a CLI tool, where:
- the 'puzzle inputs' should be piped in.
    - optionally, a file can be specified with `--input=path/to/input.file`
- the args specify which day / part the input is parsed to solve.
- the final result will be the only output (ready to be piped into another tool?).


### Example usage:
```
cargo build
cat input/day-01 | ./target/debug/advent day01 part1
{correct answer for day 1 part 1}

cargo run day01 part2 --input=input/day-01
{correct answer for part 2}
```
