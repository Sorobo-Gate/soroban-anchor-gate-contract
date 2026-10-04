# SorobanAnchor Gate Contract

SorobanAnchor Gate is an open-source decentralized middleware and milestone escrow smart contract connecting Stellar's WebAssembly smart contracts (Soroban) with the Stellar Anchor Network (SEPs).

---

## 📚 Documentation Index

- [01. Introduction & Overview](docs/01-introduction.md) — Real-world problem statement and high-level architecture diagram.
- [02. Protocol Mechanics & State Machine](docs/02-protocol-mechanics.md) — State transition rules, escrow lifecycle, and fee calculations.
- [03. Smart Contract Reference](docs/03-contract-reference.md) — Function signatures, authorization parameters, events, and errors.
- [04. Architecture & Relayer Guide](docs/04-app-and-relay-guide.md) — Monorepo structure and relay daemon configuration.
- [05. Drips Wave Contribution Guide](docs/05-contributing.md) — Guidelines for submitting PRs and earning points.

---

## 🚀 Quick Start

### Build Contracts

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests

```bash
cargo test
```

---

## ⚖️ License

Distributed under the Apache 2.0 License. See [`LICENSE`](LICENSE) for details.