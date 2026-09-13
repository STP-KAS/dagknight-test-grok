# dagknight-test-grok

Living notebook that helps **Kaspa core** ship DAGKnight without touching consensus rules from outside R&D.

**Not official. Not a spec. Not mainnet.**  
Upstream is [kaspanet/rusty-kaspa `dagknight`](https://github.com/kaspanet/rusty-kaspa/tree/dagknight). KIP-2 is still **Proposed**. Mainnet runs GHOSTDAG.

This repo exists because Sutton’s July 2025 bundle is still the map, and the work that *can* be done from outside the protocol inner loop was sitting unwritten.

---

## What this desk actually did

| When | What | Why it helps core |
|------|------|-------------------|
| 11 Sep | Cloned `dagknight`, mapped every `TODO[DK]`, reviewed open PRs against the paper | So helpers stop sending rank/UMC/tie-break PRs |
| 11 Sep | Wrote two e2e tests (integration fixture + pipeline diamond) | Fills coderofstuff’s empty `dagknight_test` TODO |
| 11 Sep | Spec’d KIP-2 confirmation RPC (client supplies `Δ`, node never picks `k`) | The unstarted KIP-2 deliverable |
| 11 Sep | Spec’d reverse-MEV kickback + miner oracle-vote **sockets** (not markets) | Sutton’s “apply the remedy before the blow” |
| 11 Sep | Logged Sutton’s parallel-DeFi post in full | App-layer dual of DK: order related sub-series, don’t serialize the world |
| 13 Sep | Rechecked GitHub + masterfile X handles | **#1127 marked ready for review 12 Sep**; last `dagknight` **commit still 8 Sep** |

Daily Grok job (09:00 Europe/Brussels) appends `log/` when something moves.

---

## Help core — do this, not that

**Do (outsider-safe):**

1. Land `tests/dagknight_test.rs` + `tests/dagknight_pipeline_e2e.rs` as **one commit** on `dagknight`, not `master`.
2. Tighten [#1124](https://github.com/kaspanet/rusty-kaspa/pull/1124) asserts (`cascade_flips > 0` on withheld-side-dag).
3. Parent-shuffle property test for [#1104](https://github.com/kaspanet/rusty-kaspa/pull/1104) — `tests/parent_shuffle.rs`.
4. Confirmation-policy RPC — `docs/01-kip2-confirmation-policy.md` + `tests/confirmation_policy.rs` types.
5. TN13 docs / seeder when they actually want a public v1.

**Do not:**

- Drive [#1127](https://github.com/kaspanet/rusty-kaspa/pull/1127) (bounded UMC, paper-level consensus change; FreshAir’s security note is still pending).
- Rewrite the executor (#1104), SSAV2 (#1121/#1122), or LCCA (#1103).
- Mix 100 BPS into DK (Sutton 23 Aug 2026).
- Treat client confirmation `Δ` as observed RTT (paper §1.4, Pass–Shi).
- Add a global DeFi sequencer opcode. That *is* the bottleneck Sutton argued against on 11 Sep.

Playbook: [docs/07-help-core.md](docs/07-help-core.md)

---

## Layout

| Path | What |
|------|------|
| [docs/00-status.md](docs/00-status.md) | Code map, TODOs, TN13, v0→v2 |
| [docs/01-kip2-confirmation-policy.md](docs/01-kip2-confirmation-policy.md) | Wallet confirmation RPC |
| [docs/02-l1-hooks-mev-oracles.md](docs/02-l1-hooks-mev-oracles.md) | Kickback + oracle-vote sockets |
| [docs/03-open-prs.md](docs/03-open-prs.md) | Open PRs: leave vs help |
| [docs/04-first-code-change.md](docs/04-first-code-change.md) | How to land the e2e tests |
| [docs/05-testnet13.md](docs/05-testnet13.md) | TN13 operator notes |
| [docs/06-partitioned-defi.md](docs/06-partitioned-defi.md) | Sutton 11 Sep: DeFi is parallel |
| [docs/07-help-core.md](docs/07-help-core.md) | Ranked next actions |
| [handles.md](handles.md) | X + GitHub watch list (from kaspa-master-file §5, DK-filtered) |
| [tests/](tests/) | Patches for the `dagknight` branch |
| [ideas/inbox.md](ideas/inbox.md) | Dated ideas |
| [log/](log/) | Dated findings |

Sutton bundle (17 Jul 2025): **DK + based ZK L1↔L2**, same HF window, plus cheap L1 sockets for reverse-MEV and miner oracle votes. 10 Aug 2026: *“The next main effort is DK.”* ZK/Toccata/Silver/Argent went first.

## Pins (rechecked 13 Sep 2026)

- Last `dagknight` commit: **8 Sep** (`#1125` UMC prune 0xFF)
- [#1127](https://github.com/kaspanet/rusty-kaspa/pull/1127) **ready for review 12 Sep** — bounded UMC `k⁴`; security write-up still “this week”
- KIP-2 Proposed. TN13 params exist, no DNS seeds, not a public launch
- Masterfile of pins: [STP-KAS/kaspa-master-file](https://github.com/STP-KAS/kaspa-master-file) §5 handles

## License

MIT for notes and patches. DAGKnight protocol remains Sutton & Sompolinsky ([eprint 2022/1494](https://eprint.iacr.org/2022/1494.pdf)).
