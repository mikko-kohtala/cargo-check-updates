# cargo-check-updates (ccu)

Upgrade your Cargo.toml dependencies to the latest versions, regardless of existing version constraints.

Similar to [npm-check-updates](https://github.com/raineorshine/npm-check-updates) for Node.js.

## Installation

```bash
make install
```

This installs both `ccu` and `cargo-check-updates` binaries.

## Quick Start

Check for available updates:
```bash
ccu
```

Upgrade all dependencies:
```bash
ccu -u
```

Interactive mode:
```bash
ccu -i
```

## Development

Build the project:
```bash
make build
```

Run checks and tests:
```bash
make dev
```

See all available commands:
```bash
make help
```

## Features (Planned)

- Upgrade dependencies beyond semver constraints
- Color-coded output (red=major, cyan=minor, green=patch)
- Interactive mode for selective upgrades
- Doctor mode to identify breaking changes
- Workspace support
- Flexible filtering by package name

## License

MIT
