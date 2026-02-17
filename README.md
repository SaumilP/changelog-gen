# 🚀 changelog-gen

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
$> cargo install changelog-gen
```

## 🔧 Usage

```bash
$> changelog-gen --release 1.2.0 --conventional --github
```

### Multi-Command CLI

```bash
changelog-gen init
changelog-gen generate
changelog-gen release --bump minor
changelog-gen self-update
```

* Run with structured logs:

```bash
RUST_LOG=info ./changelog-gen
```

Or JSON logs:
```bash
RUST_LOG=debug ./changelog-gen --json-logs
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
