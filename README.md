# sentinel-chain

**Solana attestation-anchoring program for Sentinels.** Encodes the borsh instruction + state schema for anchoring sha256 digests of [`sentinel-core`](https://github.com/Sentinels-Today/sentinel-core) attestation claims on-chain.

[![ci](https://github.com/Sentinels-Today/sentinel-chain/actions/workflows/ci.yml/badge.svg)](https://github.com/Sentinels-Today/sentinel-chain/actions/workflows/ci.yml)
![license](https://img.shields.io/badge/license-Apache--2.0-blue)
![rust](https://img.shields.io/badge/rust-1.75%2B-orange)

## What's here

- `state.rs` — `AttestationAnchor` PDA layout (borsh-serialized), versioned and validated
- `instruction.rs` — `SentinelInstruction::{AnchorAttestation, CloseAttestation}` enum
- `program.rs` — Solana entrypoint, gated behind the `solana` feature so host tooling builds cleanly without the Solana SDK
- CI: ubuntu/macos/windows fmt + clippy + tests (host build with default features)

PDA seeds: `["anchor", subject_did, claim_digest]`.

## Build

Host (CI, off-chain tooling):

```sh
cargo build
cargo test
```

On-chain (Solana BPF / SBF):

```sh
cargo build-sbf --features solana
```

The on-chain program ID placeholder is `Sent1nelChain111111111111111111111111111111`. Replace before deploying.

## License

Apache-2.0 — see [LICENSE](./LICENSE).
