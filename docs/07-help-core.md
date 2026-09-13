# Help core build DAGKnight

Ranked. One PR at a time. Mention [t.me/kasparnd/11027](https://t.me/kasparnd/11027).

## Now (no consensus rules)

| # | Work | Upstream target | Status here |
|---|------|-----------------|-------------|
| 1 | Fill empty `dagknight_test` | `testing/integration/src/consensus_integration_tests.rs` | `tests/dagknight_test.rs` |
| 2 | Pipeline diamond e2e | `consensus/src/pipeline/virtual_processor/tests.rs` | `tests/dagknight_pipeline_e2e.rs` |
| 3 | Parent-shuffle vs #1104 | new test next to `protocol.rs` | `tests/parent_shuffle.rs` |
| 4 | Tighten #1124 | simpa adversary asserts | idea in inbox; do on the PR |
| 5 | Delete stale UMC `todo!()` comment | `protocol.rs` line ~335 | comment-only nit |
| 6 | Confirmation policy types | new RPC, opcode 154 | `tests/confirmation_policy.rs` + doc 01 |

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

```
git clone -b dagknight --single-branch https://github.com/kaspanet/rusty-kaspa.git
# apply tests/dagknight_test.rs  (replace the empty fn)
# apply tests/dagknight_pipeline_e2e.rs (append)
git commit -m "[DK] Fill dagknight_test e2e from GD DAG fixtures"
# PR against kaspanet:dagknight
```

Needs LLVM (`libclang`) + `protoc` to compile on Windows.
