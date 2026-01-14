# MathCAT: Math Capable Assistive Technology

<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">
is a library that supports conversion of MathML to:


* Speech strings (in several languages) with embedded speech engine commands
* Braille (Nemeth, UEB Technical, CMU, and many others)
* Navigation of math (in multiple ways including overviews)


There are four related projects that make use of MathCAT:
- [MathCATDemo](https://nsoiffer.github.io/MathCATDemo/) -- an online demonstration of some of what can be done with MathCAT
- [A python interface for MathCAT](https://github.com/NSoiffer/MathCATForPython) -- used by a [MathCAT NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html).
- [A C/C++ interface for MathCAT](https://github.com/NSoiffer/MathCATForC)
- [A Java interface for MathCAT](https://github.com/mwhapples/MathCAT4J) (thanks to Michael Whapples for working on that)

MathCAT is used in many assistive technologies including NVDA and JAWS.

For more information, see the [full documentation](https://nsoiffer.github.io/MathCAT/).

## Test Coverage

This section explains test coverage with `llvm-cov` and `grcov` on _MacOS_.

Using other operating systems should also work with [grcov](https://github.com/mozilla/grcov), 
but may require some tweaks regarding LLVM, paths, etc.

```bash
# One-time setup
rustup component add llvm-tools-preview
cargo install grcov

export LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw"
RUSTFLAGS="-Cinstrument-coverage" cargo test
# Example with a single test:
# RUSTFLAGS="-Cinstrument-coverage" cargo test Languages::zh::tw::units::without_prefix_powers_of_2

# Generate report
grcov . \
  --source-dir . \
  --binary-path ./target/debug/deps \
  -t html \
  --branch \
  --ignore-not-existing \
  --ignore "target/*" \
  -o target/coverage/html

open target/coverage/html/index.html
```
