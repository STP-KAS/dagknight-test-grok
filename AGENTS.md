# Instructions for Grok (and humans) updating this repo

This repository is the living DAGKnight test + ideas pack for https://github.com/STP-KAS/dagknight-test-grok

When you find anything new about DAGKnight, KIP-2, TN13, Sutton's bundle (MEV reverse auctions, oracle voting, confirmation policy), rusty-kaspa `dagknight` PRs, or related Kaspa L1 work:

1. Read `log/` latest file and `ideas/inbox.md` so you do not duplicate.
2. If it is a **fact** (PR opened/merged, TODO filled, TN13 launch, paper/KIP change): append `log/YYYY-MM-DD.md` and patch `docs/00-status.md` / `docs/03-open-prs.md` if the map changed.
3. If it is an **idea**: append a dated bullet under `ideas/inbox.md`. Do not delete old ideas.
4. If it is **test/code**: put a patch under `tests/` and mention it in `docs/04-first-code-change.md`.
5. Commit with a short message (`log: …`, `idea: …`, `docs: …`, `test: …`) and push to `origin/main`.

Do **not**:

- Change DAGKnight consensus rules in this repo (rank, UMC, tie-break, SSAV2).
- Mix 100 BPS into DK notes as if they were the same fork.
- Treat client confirmation `Δ` as observed RTT (paper §1.4).
- Overwrite history; append.

Sources to check:

- https://github.com/kaspanet/rusty-kaspa/pulls?q=is%3Apr+dagknight
- https://github.com/kaspanet/rusty-kaspa/commits/dagknight
- https://github.com/kaspanet/kips/blob/master/kip-0002.md
- https://x.com/michaelsuttonil
- https://x.com/coderofstuff_
- https://kaspa.news/articles
