# Ideas inbox

Append dated bullets. Do not delete. Promote a bullet into `docs/` when it is specified enough to PR.

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
