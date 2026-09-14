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

## Public v1 seeder — wait until core asks

`TESTNET13_PARAMS.dns_seeders` is `&[]` on purpose (`consensus/core/src/config/params.rs`). coderofstuff's first iteration is an isolated history, not a public launch. Filling seeders now would look like a launch they have not announced.

When they actually want a public v1, the outsider-safe docs/seeder PR is:

1. Core names at least one DNS seeder they control (do not invent a hostname).
2. Add it to `TESTNET13_PARAMS.dns_seeders` only.
3. Document ports in `docs/testnet13.md` + `dagknight_activation` in `docs/override-params.md` (both missing upstream).
4. Dedicated ports already exist: p2p **16711**, grpc 16710, borsh 17710, json 18710 (`#1120`).
5. Keep mainnet / TN10 `dagknight_activation = never()`. Keep the `kaspad` mainnet panic until the branch is no longer experimental.
6. Do **not** copy mainnet DNS seeders. TN13 genesis is its own coin identity (`DAGKNIGHT` + `TN13, Launch 1` in the coinbase payload).

Until that request: empty seeders stay empty. This notebook will not add one.
