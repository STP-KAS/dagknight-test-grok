# Ideas inbox

Append dated bullets. Do not delete. Promote a bullet into `docs/` when it is specified enough to PR.

## 2026-09-14

- **Done (landed, not merged):** e2e tests as one commit on `dagknight` → [#1131](https://github.com/kaspanet/rusty-kaspa/pull/1131).
- **Done (landed, not merged):** parent-shuffle property test → [#1132](https://github.com/kaspanet/rusty-kaspa/pull/1132), also commented on #1104.
- **Done (review, not a competing PR):** #1124 withheld-side-dag must assert `total_cascade_flips > 0`. Comment + `tests/pr1124_withheld_cascade_flips.patch`.
- **Done (types, not opcode):** confirmation-policy crate compiles; opcode 154 still free; node still must not pick Δ.
- **Done (wait):** TN13 DNS seeder stays empty. Public-v1 checklist is in `docs/05-testnet13.md`. Do not invent a hostname.
- **Intel, do not copy:** Hoosat Network's Go `HTND` DAGKnight (fork of kaspad) is a live alt-path, not Kaspa core. Documented deviations vs paper: `g(k)=k` instead of `⌊√k⌋`, recursion from global tips, omitted `reps_G(P)`, hash-lex anticone order. Kaspa rusty-kaspa `dagknight` is the pin. See `log/2026-09-14.md`.

## 2026-09-11

- **Parent-shuffle property test for #1104.** `dagknight(parents)` vs `dagknight(shuffled(parents))` must return the same selected parent. If `sort_unstable()` on agreement groups is protocol-equivalent, this passes; if it masks a bug, this fails. Smallest test that should block merging the executor rewrite.
- **Directional UMC counters for #1127.** coderofstuff asked for `baseline T / cascade F` vs `F / T` separately. Needed before bounded-DK can be compared to paper Alg. 6.
- **Reserved 64 bytes in coinbase payload** in the DK HF even if MEV/oracle KIPs miss the date. Avoids a second header-layout fork. See `docs/02-l1-hooks-mev-oracles.md`.
- **`GetTransactionConfirmationPolicy` as KIP-2 API**, opcode 154. Node never picks `Δ`. Wallet profiles: retail-fast / exchange / settlement. See `docs/01-kip2-confirmation-policy.md`.
- **Tighten #1124 asserts** so withheld-side-dag requires `total_cascade_flips > 0`, not only `total_calls`. Honest simpa still barely conflicts.
- **TN13 DNS seeder** when v1 public testnet is actually wanted. Empty `dns_seeders` is correct for the current “not a launch” params.
- **Export `DagknightCounters` on `GetMetrics`** (replace `CustomMetricValue::Placeholder`) so simpa/devnet operators can see cascade/checkpoint hit rate without a debugger.
- **Stale comment delete:** `protocol.rs` still says `TODO[DK]: Implement full UMC cascade voting after coloring` but cascade is already called. Comment-only nit, good first PR.
- **IBD JSON coloring vs topology** (`testing/integration/src/common/json.rs`) — parse two GD blobs when present so DK IBD fixtures can exist.
- **Do not** couple 100 BPS research into the DK testnet. Sutton 23 Aug 2026: wrong complexity mix.
- **Partitioned-DeFi contention bench on TN10 (no DK required).** Two Argent/covenant apps: one shared vault vs N replica vaults with a bounded gap. Metric is rejected writes and time-to-gap, not TPS. If shared-state saturates at ~1 writer/block, Sutton’s 11 Sep 2026 hypothesis is visible before the HF. Promote: `docs/06-partitioned-defi.md`.
- **Replica-merge events are the DK-related sub-series.** Confirmation policy (client `Δ`) is how a replica decides the gap is closed. Do **not** add a global DeFi sequencer opcode in the DK HF — that *is* the bottleneck he is arguing against.
- **vProgs write-up hole (Sutton, same thread):** if DeFi truly must go through shared-state bottlenecks, ZK cannot save scalability. Track that post; don’t design vProgs as “one shared actor + proofs”.
