# Help core build DAGKnight

Ranked. One PR at a time. Mention [t.me/kasparnd/11027](https://t.me/kasparnd/11027).

## Now (no consensus rules)

| # | Work | Upstream target | Status here |
|---|------|-----------------|-------------|
| 1 | Fill empty `dagknight_test` | `testing/integration/src/consensus_integration_tests.rs` | **PR [#1131](https://github.com/kaspanet/rusty-kaspa/pull/1131)** (14 Sep, `dagknight`) |
| 2 | Pipeline diamond e2e | `consensus/src/pipeline/virtual_processor/tests.rs` | same commit as #1131 |
| 3 | Parent-shuffle vs #1104 | `protocol.rs` tests | **PR [#1132](https://github.com/kaspanet/rusty-kaspa/pull/1132)**; comment on #1104 |
| 4 | Tighten #1124 | simpa adversary asserts | comment + `tests/pr1124_withheld_cascade_flips.patch` |
| 5 | Delete stale UMC `todo!()` comment | `protocol.rs` line ~335 | comment-only nit, still open |
| 6 | Confirmation policy types | new RPC, opcode 154 | `tests/confirmation_policy.rs` (8 unit tests pass locally) + doc 01 |

## Wait on core

| PR | Why wait |
|----|----------|
| #1127 | Bounded UMC. Ready for review **12 Sep**. FreshAir: formal security note “hopefully this week”. Continuation: UMC from any viewpoint (checkpoints+diffs vs versioned SMT). |
| #1104 | Executor rewrite. Need shuffle test before merge. |
| #1121 #1122 | SSAV2 drafts; rebase after #1123. |
| #1103 | LCCA hull. Coordinate with #1104. |

## Why this ranking

coderofstuff split DK into (1) protocol (2) wiring. Wiring (DAA, coinbase, IBD, pruning, TN13 `ForkActivation`) is already on the branch. The empty e2e is the cheapest lock on (2). #1124 is the cheapest lock on (1)’s attack shapes. #1127 is the only live **paper change**; outsiders should not steer it.

Sutton 11 Sep: related sub-series still need order; the rest of DeFi should not serialize. Confirmation RPC (client `Δ`) is how wallets use DK without re-hardcoding `k`. That is KIP-2’s second deliverable. Protocol inner loop is not.

## How to send a test PR

Sent 14 Sep 2026 as [#1131](https://github.com/kaspanet/rusty-kaspa/pull/1131) against `kaspanet:dagknight` from `STP-KAS:dk-fill-dagknight-test-e2e`.

This Windows box still cannot compile rusty-kaspa (`libclang.dll` / `protoc` missing). CI on the PR is the compile. Confirmation-policy types compile here: `cargo test` in this repo.
