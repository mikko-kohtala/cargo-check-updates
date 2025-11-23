# cargo-check-updates (ccu)

Upgrade your Cargo.toml dependencies to the latest versions. Like [npm-check-updates](https://github.com/raineorshine/npm-check-updates) for Rust.

## Installation

```bash
cargo install --path .
```

Or using make:
```bash
make install
```

## Usage

**Check for updates** (dry-run):
```bash
ccu
```

**Upgrade dependencies**:
```bash
ccu -u
```

**Filter specific packages**:
```bash
ccu "serde*"
ccu --reject "test-*"
```

**Specify Cargo.toml path**:
```bash
ccu --manifest-path path/to/Cargo.toml
```

## Example Output

```
Checking Cargo.toml

 clap         4.0.0  →  4.5.53
 serde      1.0.100  →  1.0.228
 tokio       1.20.0  →  1.47.2

Run ccu -u to upgrade Cargo.toml
```

## Features

✅ Upgrades beyond semver constraints (always suggests latest)
✅ Color-coded output (red=major, cyan=minor, green=patch)
✅ Parallel crates.io queries for speed
✅ Preserves version operators (`^1.0` → `^2.0`)
✅ Handles all dependency sections (dependencies, dev-dependencies, build-dependencies)
✅ Filter/reject patterns for selective updates
✅ Preserves TOML formatting

🚧 Interactive mode (coming soon)
🚧 Doctor mode with test validation (coming soon)

## Development

```bash
make build      # Build debug version
make release    # Build release version
make test       # Run tests
make clippy     # Run linter
make dev        # Format + lint + test
```

## License

MIT
