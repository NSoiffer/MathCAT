# Developer Guide

This guide provides technical information for developers working on the MathCAT codebase.

## Prerequisites

To develop MathCAT, you need to have Rust installed. If you haven't already:

1. [Download and install Rust](https://www.rust-lang.org/tools/install)
2. Clone the MathCAT repository
3. Open the project directory in your IDE

## Working with Cargo

Cargo is Rust's build system and package manager. Here are the essential commands you'll use:

### Building the Project

```bash
# Build the project in debug mode
cargo build

# Build the project in release mode (optimized)
cargo build --release
```

### Running the Project

```bash
# Run the main executable
cargo run

# Run with specific arguments
cargo run -- <args>
```

### Managing Dependencies

Dependencies are defined in `Cargo.toml`. Cargo automatically downloads and manages them.

```bash
# Update dependencies to their latest compatible versions
cargo update
```

## Testing

Testing is crucial for maintaining code quality and ensuring that changes don't break existing functionality.

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name
```

### Writing Tests

Tests in MathCAT verify that MathML expressions produce the expected speech output. Example:

```rust
#[test]
fn test_simple_fraction() {
    let expr = "<math>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "1 half");
}
```

### Test Coverage

Test coverage helps identify which parts of the code are exercised by tests and which parts need more testing.

<details>
<summary>Using grcov on macOS</summary>

This approach uses `llvm-cov` and `grcov` to generate test coverage reports. Other operating systems should also work with [grcov](https://github.com/mozilla/grcov), but may require some adjustments regarding LLVM paths and configuration.

**One-time setup:**

```bash
# Install required components
rustup component add llvm-tools-preview
cargo install grcov
```

**Generate coverage report:**

```bash
# Set environment variable for profiling data
export LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw"

# Run tests with coverage instrumentation
RUSTFLAGS="-Cinstrument-coverage" cargo test

# Example: Run a single test
# RUSTFLAGS="-Cinstrument-coverage" cargo test Languages::zh::tw::units::without_prefix_powers_of_2

# Generate HTML report
grcov . \
  --source-dir . \
  --binary-path ./target/debug/deps \
  -t html \
  --branch \
  --ignore-not-existing \
  --ignore "target/*" \
  -o target/coverage/html

# Open the report in your browser
open target/coverage/html/index.html
```

</details>

**Alternative: IDE Integration**

Many Rust IDEs provide built-in test coverage support, like RustRover or VSCode.