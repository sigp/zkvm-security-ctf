# Setup

## Rust

```bash
rustup toolchain install stable
rustup default stable
```

## SP1

```bash
curl -L https://sp1up.succinct.xyz | bash
sp1up
cargo install --locked cargo-prove
```

## Foundry

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

## Verification

Use per-challenge commands documented in each challenge `README.md`.

Rust checks are always run from challenge-local workspace roots:

```bash
cargo test --workspace
```

Foundry challenges additionally run:

```bash
forge test
```
