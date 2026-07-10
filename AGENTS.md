# OpenCode Agent Instructions

## Workspace Structure (Code Kata Style)
This is a Rust practice repository. To keep it clean and fast to compile, we use a single library crate with separate modules for each kata, rather than a heavy Cargo workspace:
- **Root Crate**: Defined in `/Users/agand/Workspace/practice-rust/Cargo.toml`
- **Modules**: Located in `src/<kata_name>.rs` or `src/<kata_name>/mod.rs`
- **Entry point**: Register modules in `src/lib.rs` (e.g., `pub mod fizzbuzz;`)

## Developer Commands & Verification
Always verify changes in the following sequence:
1. Format: `cargo fmt --all -- --check`
2. Lint: `cargo clippy --all-targets -- -D warnings`
3. Test: `cargo test`

To test a single kata, run:
```bash
cargo test <kata_name>
```

## Unit Testing Conventions
- Keep unit tests inside the same file as the implementation in a nested `tests` module:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_example() {
          // test logic
      }
  }
  ```
- Use `io::stdout().flush()?` when writing interactive CLI katas (e.g., guessing game) to ensure prompts appear before reading input.
