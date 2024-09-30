# 🦀 Rust LeetCode Solutions

A bunch of LeetCode solutions in Rust.


## How To Use

### 1. Clone the Repository

```bash
git clone https://github.com/BenSparksCode/rust-leetcode.git
cd rust-leetcode
```

### 2. Workspace Setup

This project is structured as a Rust workspace. Each problem is a separate crate inside the problems/ folder, following the naming convention `pXXXX_problem_name`.

For example:

```
problems/
├── p0001_two_sum
├── p0002_add_two_numbers
└── p0003_longest_substring_without_repeating_characters
```

### 3. Add a New Problem

To add a new LeetCode problem, first make sure you're in the `rust-leetcode/problems/` directory.

```bash
cd problems
```

Then, run the following command, replacing `XXXX` with the problem number and `problem_name` with the name of the problem:

```bash
cargo new pXXXX_problem_name --lib
```

This will create a new Rust library crate with the specified name. You can then implement the solution in the `src/lib.rs` file.

### 4. Run the Tests

There are unit tests for each problem. You can run them either per individual problem or all at once.

**Run tests for all problems:**

```bash
cargo test
```

**Run tests for a specific problem:**

```bash
cargo test -p p0001_two_sum
```