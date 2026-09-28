# 🦀 Chapter 11: Automated Testing Mastery

## 🏗️ Architecture Vision
This repository contains the hardcore practice vectors for Chapter 11 of the Official Rust Book. It serves as a definitive proof of mastery over Rust's robust automated testing engine. Every single program is engineered to be 100% memory-safe, crash-proof, and enterprise-ready.

## 🧠 Core Arsenal Deployed
- **Unit Testing:** In-file `#[cfg(test)]` modules testing isolated logic.
- **Integration Testing:** External `tests/` directory architecture mimicking real-world client usage.
- **Macro Assertions:** Advanced usage of `assert!`, `assert_eq!`, and `assert_ne!`.
- **Crash Verification:** Explicit panic handling using `#[should_panic]` with exact message matching.
- **Result-Based Tests:** Handling scalable tests using `Result<T, E>`.
- **Test Runner Flags:** Granular control over threads, ignored tests, and stdout filtering.

## 📂 Test Vectors Breakdown

1. **`01_basic_math_assertions`**: Validates foundational equality and inequality using `assert_eq!` and `assert_ne!`.
2. **`02_rectangle_can_hold`**: Implements custom Struct testing utilizing boolean `assert!` evaluations.
3. **`03_custom_error_messages`**: Demonstrates precision debugging with custom formatted failure messages.
4. **`04_panic_verification`**: Forces and validates controlled application crashes using `#[should_panic]`.
5. **`05_precise_panic_matching`**: Highly strict panic validation matching exact string outputs using the `expected` parameter.
6. **`06_result_based_tests`**: Scales tests gracefully without panicking, returning `Ok(())` or `Err()`.
7. **`07_ignored_heavy_computation`**: Optimizes the test suite by bypassing resource-heavy tests via the `#[ignore]` attribute.
8. **`08_private_function_testing`**: Breaches module privacy rules (by design) to test internal unexposed functions using `use super::*;`.
9. **`09_integration_test_setup`**: Establishes a true external client-testing environment via the root `tests/` directory.
10. **`10_common_module_trap`**: Secures shared testing logic within `tests/common/mod.rs` to prevent Rust from executing helper code as standalone tests.

## 🚀 Execution Command
To run the entire test suite and verify the integrity of the codebase, execute:
```bash
cargo test