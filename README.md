# dagknight-test-grok

Living notebook for **Kaspa DAGKnight** tests, ideas, and the broader Sutton bundle (DK + confirmation policy + reverse-MEV + oracle votes).

Not consensus. Not an official Kaspa repo. Notes and outsider-safe patches so core R&D is easier to help.

**Upstream:** [kaspanet/rusty-kaspa `dagknight`](https://github.com/kaspanet/rusty-kaspa/tree/dagknight) · [KIP-2](https://github.com/kaspanet/kips/blob/master/kip-0002.md) · [paper](https://eprint.iacr.org/2022/1494.pdf)

**R&D:** [t.me/kasparnd/11027](https://t.me/kasparnd/11027) · Sutton [@michaelsuttonil](https://x.com/michaelsuttonil) · coderofstuff [@coderofstuff_](https://x.com/coderofstuff_)

## Layout

| Path | What |
|------|------|
| [docs/00-status.md](docs/00-status.md) | Code map, TODOs, TN13, v0→v2 |
| [docs/01-kip2-confirmation-policy.md](docs/01-kip2-confirmation-policy.md) | Wallet confirmation RPC (client supplies Δ/β/ε) |
| [docs/02-l1-hooks-mev-oracles.md](docs/02-l1-hooks-mev-oracles.md) | Reverse-MEV kickback + miner oracle-vote sockets |
| [docs/03-open-prs.md](docs/03-open-prs.md) | Open PRs: leave to core vs help |
| [docs/04-first-code-change.md](docs/04-first-code-change.md) | How to land the e2e tests upstream |
| [docs/05-testnet13.md](docs/05-testnet13.md) | TN13 operator notes |
| [docs/06-partitioned-defi.md](docs/06-partitioned-defi.md) | Sutton 11 Sep 2026: DeFi is parallel; don’t serialize all state |
| [tests/](tests/) | Patches for rusty-kaspa (`dagknight` branch) |
| [ideas/inbox.md](ideas/inbox.md) | New ideas (append, don't overwrite) |
| [log/](log/) | Dated findings as DK moves |

## Sutton's bundle (17 Jul 2025)

One hard fork: DAGKnight + based ZK L1↔L2. Same window, cheap L1 sockets:

1. Reverse MEV auctions (canonical kickback + deterministic bid ranking)
2. Oracle voting (miner attestations each round)

10 Aug 2026: *"The next main effort is DK."* DK ≠ higher BPS.

## How this repo stays current

Grok watches DAGKnight PRs, Sutton/coderofstuff posts, and kaspa.news. New facts go in `log/YYYY-MM-DD.md`. New ideas go in `ideas/inbox.md`. Status docs get patched when the map changes.

If you are Grok in a later session: read `AGENTS.md`, then update and push.

## License

Notes and patches: MIT. DAGKnight protocol remains Sutton & Sompolinsky (paper). Do not treat this as a consensus specification.
