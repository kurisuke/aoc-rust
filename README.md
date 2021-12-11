# Advent of Code, implemented in Rust

## How to run with your AoC account

This suite can auto-download your personalized AoC input if you provide it with the session cookie from your logged in browser session.

1. Log in to [Advent of Code](https://adventofcode.com) in your browser.
2. Get the value of the cookie named "session" from the browser developer tools.
3. Store the value in your [dotenv](https://crates.io/crates/dotenv), e.g. in the file `.env` in the repo root path.

```
AOC_SESSION=5361[...]1448
```

## Command-line usage

The code should be run with the `cargo run --release` command from the root path of the repo.

Examples:

Run the code for the latest day (no arguments):

```
cargo run --release
```

Run the code for a specific day of the most recent year:

```
cargo run --release 11
```

Run the code for a specific year / day:

```
cargo run --release 2016 11
```

Run all implemented days for a single year:

```
cargo run --release 2016 all
```

## Code structure

A solution for each day is implemented in a single Rust source file: `yearxxxx/src/dayxx.rs`.

Re-used code has been moved to the subcrate `util`.

## Tests

AoC example input is implemented as unit tests in the respective file. 

