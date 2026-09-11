# Testnet-13 (DAGKnight), operator notes

Not a public launch. Params landed in PR #1120 (6 Sep 2026). No DNS seeders.

## Run a node (once you have a `dagknight` build)

```
kaspad --testnet --netsuffix=13
```

- P2P default port: **16711** (`consensus/core/src/network.rs`)
- Genesis payload includes `DAGKNIGHT` + `TN13, Launch 1`
- `dagknight_activation = ForkActivation::always()` — DK from genesis
- Mainnet / TN10 / simnet / devnet: DK **off**
- This branch's `kaspad` **panics on mainnet** on purpose (`kaspad/src/daemon.rs`)

Override JSON already accepts `dagknight_activation`; it is missing from `docs/override-params.md` in the repo. Add it there in a docs-only PR.

## Wallet maturity (still GD-era)

TN13 wallet params still use DAA maturity: user 100, coinbase 1000, stasis 500 (`wallet/core/src/utxo/settings.rs`). Wallet-core confirms UTXOs by `virtual_daa_score >= block_daa_score + maturity`. It does **not** subscribe to `SinkBlueScoreChanged`. KIP-2 confirmation RPC (doc 01) is how that should change.

## What TN13 is for

- v1 of coderofstuff's v0/v1/v2 plan
- Isolated history (own genesis, not a copy of mainnet coins)
- Exercise DAA, coinbase, IBD, pruning **wiring** around DK parent selection

It is not a promise that confirmation times match the paper's optimistic bound.
