# Open DAGKnight PRs — who should touch them

Reviewed 11 Sep 2026 against the paper (Algs. 2–6) and KIP-2. Rechecked 14 Sep: last **upstream** `dagknight` commit still 8 Sep (`#1125`). Outsider PRs opened today: **#1131** e2e, **#1132** parent-shuffle. #1127 still ready-for-review.

## Leave to core

### #1127 bounded UMC (freshair18) — **the only paper-level consensus change**

**Ready for review 12 Sep 2026** (was draft). Replaces unbounded Alg. 6 with: reject coloring if a blue at depth ≥ `k⁴+1` from the merger has a negative vote, without finishing the walk to conflict genesis.

FreshAir 12 Sep: batching, better depth-limit search, event-order polish, rough runtime analysis are in. **Formal security document still pending this week.** Follow-up PR (not this one): UMC from any viewpoint — (a) pruned checkpoints + diffs, or (b) versioned cells like SMT (`@biryukovmaxim`).

- Author admits this is **not** vanilla DK. Security note is a TODO.
- Intended error: false *reject* (slower, safer). False *accept* is supposed to be impossible — **not proven**.
- Baseline-vs-cascade panic was removed. Disagreement counters are incomplete.
- `unprocess_mergesets` is unused (reorg path unproven).
- Checkpoints clone `events_diff` — store bloat.

Do **not** bikeshed `k.pow(4)`. Wait for Sutton/freshair18 write-up.

Edge help only: directional counters (`baseline T/cascade F` vs `F/T`) that coderofstuff already asked for.

### #1104 executor rewrite (biryukovmaxim)

Changes requested 3× by coderofstuff. Index-based `SmallVec<u16>` conflict walk, typed `FreeSearch`/`CommittedSearch`.

Intended protocol-equivalent. Hazards:

- `sort_unstable()` on agreement groups can **mask** parent-order bugs. Paper does not sort. Shuffle-vs-`dagknight` permutation test is **[#1132](https://github.com/kaspanet/rusty-kaspa/pull/1132)** (commented on this PR 14 Sep).
- LCCA overlaps #1103.
- Differential tests vs old executor were dropped.

Leave it. Do not “help” by restyling hashes vs indices.

### #1121 / #1122 SSAV2 (D-Stacks)

Both drafted pending #1123 — **#1123 already merged**. They need rebase.

- #1121: one-pass BFS to a UTXO-qualified ancestor when DK’s SP is disqualified. Kaspa-only (paper has no UTXO). Holding `pruning_lock` across the BFS is the stall risk. No test.
- #1122: reuse SP instead of re-running DK after dropping merge-breaking parents. **Stale SP is a consensus change** unless `dagknight(new_parents).sp == old_sp`. Dirty vs `*_v2`.

Leave. After rebase, the missing test is: chain A←B←C with C DK-selected and UTXO-disqualified, B disqualified, A valid → A in tip set, no livelock.

## Ours (14 Sep 2026)

### #1131 e2e tests (STP-KAS)

One commit on `dagknight`: fill empty `dagknight_test` + pipeline diamond. Structural invariants, not GD expected selected-parents.

### #1132 parent-shuffle (STP-KAS)

`dagknight(parents)` vs 32 shuffles of four genesis children. Gate for #1104. Does not rewrite the executor.

## Outsider-friendly

### #1124 simpa adversarial scenarios (Kali123411) — **best outsider PR**

Honest simpa with uniform delay never conflicts, so UMC counters stay ~0. This manufactures paper attack shapes.

Tighten before merge:

| Scenario | PR table claims | Code actually asserts |
|----------|-----------------|------------------------|
| withheld-side-dag | cascade_flips > 0 | only `total_calls` and `voting_blocks` — **tighten posted 14 Sep** ([comment](https://github.com/kaspanet/rusty-kaspa/pull/1124#issuecomment-5659561079), patch in `tests/pr1124_withheld_cascade_flips.patch`) |
| gray-context-change | no baseline disagreement | `total_calls > 0` (panic already gone in #1127) |
| weak-shortcut | from_scratch dominates | from_scratch > 0 while both runs are checkpoint-heavy |

CI needs maintainer “Approve and run”. Follow-up: re-root weak-shortcut so scratch actually dominates.

### #1103 hull-based LCCA (biryukovmaxim)

Moves Alg. 2’s latest common chain ancestor into reachability (interval hull). Perf, not a rule change **if** hull ≡ current walk.

Coordinate with #1104 (both touch `is_chain_ancestor_of_all`). Outsider can add a property test: random parent sets, hull LCCA == backward-chain LCCA.

## Merge order (suggested)

1. #1131 (e2e wiring lock) — no protocol change
2. #1132 (parent-shuffle) then #1104 (executor)
3. #1103 (LCCA primitive) if tests show equivalence
4. #1121/#1122 rebase onto current SSAV2
5. #1124 (harness) after cascade_flips tighten — does not block protocol
6. #1127 only after the security note
