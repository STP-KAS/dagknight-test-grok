# First code change in the local clone

Two tests, same invariants. Integration one is the TODO coderofstuff left; consensus-crate one does not need `protoc`.

## 1. Integration placeholder (KIP-2 e2e TODO)

`rusty-kaspa-dagknight/testing/integration/src/consensus_integration_tests.rs` — `dagknight_test`

Uses `testdata/dags/dag0.json` … `dag5.json`.

## What it does

Same JSON DAG fixtures as `ghostdag_test` (`testdata/dags/dag0.json` … `dag5.json`), but:

- `p.dagknight_activation = ForkActivation::always()`
- `build_header_with_parents(..., is_dk_active = true)` so DAA/bits/blue_score match DK coloring
- Does **not** assert GHOSTDAG expected selected-parents (those are GD fixtures)

Asserts instead:

1. Every header inserts through the full pipeline (header + virtual)
2. Coloring selected parent ∈ direct parents
3. Topology selected parent ∈ direct parents (free GD still running)
4. Coloring blue score ≥ 1
5. Walking coloring selected-parent from the last block reaches genesis

That is the outsider-safe e2e coderofstuff left as `TODO[DK]`. It locks current DK pipeline behavior without changing rank/UMC/tie-break.

## 2. Consensus-crate pipeline e2e (runs without protoc)

`rusty-kaspa-dagknight/consensus/src/pipeline/virtual_processor/tests.rs` — `dagknight_pipeline_e2e`

Synthetic diamond + two merges (`B,C` over genesis, then `X` merging `E,F`) so rank/UMC actually execute.

## How to run

```
cd C:\Users\Remco\rusty-kaspa-dagknight
cargo test -p kaspa-consensus dagknight_pipeline_e2e -- --nocapture
cargo test -p kaspa-testing-integration dagknight_test -- --nocapture
```

This machine could not finish a build: no `libclang.dll` (rocksdb bindgen) and no `protoc` (p2p). LLVM dir exists but is empty of clang. Install LLVM + protobuf, set `LIBCLANG_PATH` and `PROTOC`, then run the commands above.

First build of this clone is long (workspace crate).

## How to turn it into an upstream PR

1. Fork `kaspanet/rusty-kaspa`, branch from `dagknight` (not `master`).
2. One commit: only the test.
3. Title: `[DK] Fill dagknight_test e2e from GD DAG fixtures`
4. Body: structural invariants, not GD expected data; TN13/activation always; does not claim paper traces.
5. Mention it in t.me/kasparnd/11027.

Do **not** mix this with confirmation RPC or MEV hooks. Separate PRs.

## Next code PRs (still no consensus rules)

1. Comment nit: delete stale `protocol.rs` “TODO[DK]: Implement full UMC cascade voting after coloring” — cascade is already called.
2. `#1124` assert tightening (`cascade_flips` for withheld-side-dag).
3. KIP-2 confirmation RPC (`01-kip2-confirmation-policy.md`) — new opcode, read-only.
4. `docs/testnet13.md` + `dagknight_activation` in `docs/override-params.md`.
5. Export `DagknightCounters` on `GetMetrics` (replace `CustomMetricValue::Placeholder`).
6. IBD JSON: parse coloring vs topology GD separately (`testing/integration/src/common/json.rs`).

## Do not send

- Tie-break `k'` range
- UMC depth bound
- SSAV2 BFS
- Executor index rewrite
- Mainnet panic removal
- 100 BPS
