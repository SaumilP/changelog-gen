# 🚀 changeloggen-cli

Production-grade Rust CLI to generate beautiful changelogs from Git history.

## ✨ Features

- Conventional Commit support
- GitHub compare links
- Semantic versioning
- crates.io ready
- CI included
- Fully testable

## 📦 Installation

```bash
$> cargo install changeloggen-cli
```

## 🔧 Usage

```bash
$> changeloggen-cli --release 1.2.0 --conventional --github
```

### Multi-Command CLI

```bash
changeloggen-cli init
changeloggen-cli generate
changeloggen-cli release --bump minor
changeloggen-cli self-update
```

* Run with structured logs:

```bash
RUST_LOG=info ./changeloggen-cli
```

Or JSON logs:
```bash
RUST_LOG=debug ./changeloggen-cli --json-logs
```


## 🧠 Example Output

## [1.2.0] - 2026-02-17

### FEAT

- add login support

### FIX

- correct API bug

## 🛠 Development

```bash
$> cargo build
$> cargo test
```

## 🤝 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md)
