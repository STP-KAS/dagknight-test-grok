# DAGKnight status (rechecked 13 Sep 2026)

Last `dagknight` **commit still 8 Sep** (`#1125`). Material delta: **#1127 marked ready for review 12 Sep**. Security write-up still pending. KIP-2 still **Proposed**. Mainnet runs GHOSTDAG. DK is on `dagknight`, gated by `ForkActivation`.

## Iterations (coderofstuff, Feb 2026)

- **v0 devnet** — end-to-end flow, even if some paper pieces are partial. Internal devnet has run.
- **v1 testnet** — TN13 params landed (PR #1120, 6 Sep 2026). No DNS seeds. Not a public launch.
- **v2 mainnet candidate** — not started.

## Four protocol components → files

Sutton's task list is still in `consensus/src/processes/dagknight/protocol.rs`.

| Component | Paper | File | State |
|-----------|-------|------|-------|
| Hierarchical conflict resolution | Alg. 2 | `protocol.rs` `dagknight()` | In |
| Rank / MkMC k-search | Alg. 3 | `rank_search.rs` | In |
| Committed vs free coloring | Alg. 5 | `manager.rs` ConflictZoneManager | In |
| Gray blocks (not representatives) | — | coloring path | In |
| Tie-break | Alg. 4 | `tie_breaking.rs` | In; checks `k-1` not the full `[⌊k/2⌋, k]` range (Sutton suggestion) |
| Incremental UMC cascade | Alg. 6 | `umc_cascade.rs`, `umc_voting.rs`, `umc_cascade_persistence.rs` | In; **bounded-DK** PR #1127 **ready for review 12 Sep** (paper change; security note pending) |

Dual coloring:

- **Committed** on the DK megachain → blue score, coinbase, confirmation counting
- **Free GHOSTDAG** → blue work / topology (DK does not maximise blue work)

## Network gating

```
MAINNET / TN10 / DEVNET / SIMNET:  dagknight_activation = never()
TESTNET13:                         dagknight_activation = always()
simpa:                             forced always() in simpa/src/main.rs
```

TN13 genesis payload includes `DAGKNIGHT` + `TN13, Launch 1`. Empty `dns_seeders`.

`kaspad` still panics on mainnet (`kaspad/src/daemon.rs`). Keep that until the branch is no longer experimental, even though params already say `never()`.

## Open TODOs that are real work

Consensus-touching (leave to Sutton / coderofstuff / PR authors):

- `tie_breaking.rs:215` — single `k'` vs paper range
- `protocol.rs:335` — comment stale; cascade **is** called. Old `CascadeContext::vote()` is still `todo!()` in unused experimental module
- `virtual_processor/processor.rs:327` — tip filter before SSAV2
- `virtual_processor/processor.rs:1921` — pruning-point activation DAA
- `pruning_processor/processor.rs:509` — pruning DK-rooted records
- `header_processor/processor.rs:307` — batch DK store inserts into `commit_header`
- `window.rs:320` — which store for ordering
- `pruning_proof/apply.rs:180` — coloring GD in proofs
- `#1127` bounded UMC (`k^4` depth) — security write-up missing

Outsider-safe:

- Integration e2e placeholder — **filled in this clone** (`dagknight_test`)
- `#1124` simpa adversarial scenarios (honest simpa barely conflicts)
- Wallet confirmation-policy RPC (KIP-2, see `01-kip2-confirmation-policy.md`)
- Remaining activation comments + TN13 seeder / ports docs
- `testing/integration/src/common/json.rs:29` — coloring vs topology in JSON tests

## Open PRs (11 Sep 2026)

| PR | Author | Touch consensus? | Outsider action |
|----|--------|------------------|-----------------|
| [#1127](https://github.com/kaspanet/rusty-kaspa/pull/1127) | freshair18 | **Yes** — bounded UMC | Read, do not drive |
| [#1124](https://github.com/kaspanet/rusty-kaspa/pull/1124) | Kali123411 | No — simpa harness | Review, extend scenarios |
| [#1122](https://github.com/kaspanet/rusty-kaspa/pull/1122) / [#1121](https://github.com/kaspanet/rusty-kaspa/pull/1121) | D-Stacks | Yes — SSAV2 | Leave |
| [#1104](https://github.com/kaspanet/rusty-kaspa/pull/1104) | biryukovmaxim | Yes — executor rewrite | Leave; 32 comments already |
| [#1103](https://github.com/kaspanet/rusty-kaspa/pull/1103) | biryukovmaxim | Reachability perf | Leave |

## Sutton caveats still in force

- DK is not “get faster.” It is 50% Byzantine + no in-protocol latency bound.
- Confirmation tracks **adversarial** recent latency the **client** assumes, not observed RTT (paper §1.4, Pass–Shi).
- Do not mix extreme BPS with DK.
- Faster total-order convergence under attack is the reason to bundle with the ZK/app layer.
