# Rust Profiling with perf - Fixing Mangled Symbols

When profiling Rust applications with `perf`, you may see mangled/cryptic symbol names instead of readable function names. This guide shows you how to fix it.

## Why Symbols Are Mangled or Missing

There are two common causes:

1. **Rust Name Mangling**: Rust mangles function names to encode additional information (generics, traits, module paths) into symbol names
2. **Missing Debug Symbols for System Libraries**: System libraries like libc, libstd, etc. may not have debug symbols installed, causing `perf` to show raw addresses or `[unknown]` for those functions

## Solution 1: Quick Fix - Use Demangling Flag

The simplest solution is to tell `perf` to demangle Rust symbols:

```bash
# Build your project
cargo build --release

# Record performance data
perf record --call-graph=dwarf ./target/release/your_binary

# View report with demangled symbols
perf report --demangle=rust
```

Alternatively, you can use:
```bash
perf report --demangle
```

## Solution 2: Add Debug Info to Release Builds

Edit your `Cargo.toml` to include debug symbols in release builds:

```toml
[profile.release]
debug = true
```

Or for smaller binary size with just line tables:

```toml
[profile.release]
debug = 1  # Line tables only, good for profiling
```

Then rebuild and profile:

```bash
cargo build --release
perf record --call-graph=dwarf ./target/release/your_binary
perf report --demangle=rust
```

## Solution 3: Best Configuration (Recommended)

For optimal profiling with readable symbols and source code mapping:

**1. Add to your `Cargo.toml`:**

```toml
[profile.release]
debug = 1           # Include line table info
strip = false       # Don't strip symbols
```

**2. Build and profile:**

```bash
# Clean build with new profile settings
cargo clean
cargo build --release

# Record with DWARF call graph for better stack traces
perf record --call-graph=dwarf ./target/release/your_binary

# View report with demangled Rust symbols
perf report --demangle=rust
```

## Solution 4: Use cargo-flamegraph (Alternative Tool)

For automatic demangling and better visualization:

```bash
# Install cargo-flamegraph (one-time)
cargo install flamegraph

# Build and profile in one command (automatically demangled)
cargo flamegraph --release

# This generates flamegraph.svg that you can open in a browser
```

## Complete Workflow Example

```bash
# 1. Configure Cargo.toml (see Solution 3)

# 2. Clean and rebuild
cargo clean
cargo build --release

# 3. Run your binary with perf recording
perf record --call-graph=dwarf --freq=997 ./target/release/your_binary [args]

# 4. View the report with demangled symbols
perf report --demangle=rust

# 5. (Optional) Generate annotation for specific functions
perf annotate --demangle=rust
```

## Additional Tips

- Use `--call-graph=dwarf` for more accurate call stacks
- Use `--freq=997` to set sampling frequency (default is 1000Hz)
- If you need to profile with different arguments: `perf record ./target/release/binary -- arg1 arg2`
- To see all samples: `perf report --demangle=rust --stdio`

## Installing Debug Symbols for System Libraries

If you see `[unknown]` or raw addresses for system functions (libc, pthread, etc.), you need to install debug symbol packages:

### Ubuntu/Debian

```bash
# Enable debug symbol repositories
echo "deb http://ddebs.ubuntu.com $(lsb_release -cs) main restricted universe multiverse" | \
  sudo tee -a /etc/apt/sources.list.d/ddebs.list

echo "deb http://ddebs.ubuntu.com $(lsb_release -cs)-updates main restricted universe multiverse" | \
  sudo tee -a /etc/apt/sources.list.d/ddebs.list

# Import the debug symbol archive signing key
sudo apt install ubuntu-dbgsym-keyring

# Update and install debug symbols
sudo apt update
sudo apt install libc6-dbg libc6-dbgsym
```

### Fedora/RHEL/CentOS

```bash
# Install debug info packages
sudo dnf debuginfo-install glibc

# Or manually install specific packages
sudo dnf install glibc-debuginfo glibc-debuginfo-common
```

### Arch Linux

```bash
# Enable debuginfod (automatic debug symbol download)
export DEBUGINFOD_URLS="https://debuginfod.archlinux.org"

# Or install debug packages manually
sudo pacman -S glibc-debug
```

### Verify Debug Symbols Are Loaded

```bash
# Check if perf can find debug info
perf report --stdio | grep -E '\[unknown\]|libc'

# If you see fewer [unknown] entries, debug symbols are working
```

## Troubleshooting

### If Rust symbols are still mangled:

1. Verify debug info is included: `cargo build --release -vv | grep debug`
2. Check that symbols exist: `nm target/release/your_binary | grep -i "your_function"`
3. Try `perf report --demangle` without `=rust`
4. Ensure perf is up to date: `perf --version`

### If you see `[unknown]` for system functions:

1. Check if debug symbols are installed: `dpkg -l | grep dbgsym` (Ubuntu/Debian)
2. Verify perf can find symbols: `perf buildid-list`
3. Try using debuginfod (automatic debug info): `export DEBUGINFOD_URLS="https://debuginfod.elfutils.org/"`
4. Check library paths: `perf report --stdio --verbose 2>&1 | grep -i "failed to open"`

### Common Issues:

- **Missing `/usr/lib/debug` directory**: Debug symbols aren't installed
- **`[unknown]` in libc functions**: Need to install `libc6-dbg` or `glibc-debuginfo`
- **Mangled Rust symbols**: Use `--demangle=rust` flag
- **Empty call stacks**: Try `--call-graph=dwarf` instead of `--call-graph=fp`
