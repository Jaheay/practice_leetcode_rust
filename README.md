# LeetCode Rust Solutions Repository

This repository contains my solutions to various LeetCode problems, written in Rust. Each problem is implemented as a module and tested using the examples and constraints provided by  LeetCode.

- **Solutions:** Organized in `src/problems/` as `problem#.rs`.
- **Reusable Code:** Common utilities (e.g., linked lists) are stored in `src/problems/common.rs`.
- **Test-Driven:** All solutions are tested with:
  - Example test cases in `#[cfg(test)] mod examples`.
  - Constraint validation in `#[cfg(test)] mod constraints`.

## Directory Structure
- `src/problems/`: Individual problem solutions (`problem#.rs`).
- `src/problems.rs`: Includes all problem modules.
- `src/problems/common.rs`: Shared utilities (e.g., linked lists).


## Tests
The repository is designed for seamless testing using Rust's `cargo test`. 

- **Run All Tests:**
  ```bash
  cargo test
  ```
  The `main.rs` ensures `cargo test` runs and verifies the results.

- **Run a Specific Problem's Tests:**
  ```bash
  cargo test problem#
  ```

## Problem Template
Each new problem starts from `src/problems/problem_template.rs`, structured as:
```rust
pub struct Solution;

impl Solution {
    pub fn solution() {
        unimplemented!();
    }
}

#[cfg(test)]
mod examples {
    use super::*;
    // #[test]
}

#[cfg(test)]
mod constraints {
    use super::*;
    // #[test]
    // #[should_panic]
}
```

## Contributing
Feel free to explore and test the solutions, or adapt the structure for your own LeetCode journey!

Happy coding! 🚀✨💖