# 2026-09-11 — Sutton: global DeFi is not sequential

- Author: Michael Sutton (@michaelsuttonil)
- Time: Fri 11 Sep 2026 00:17 UTC
- Post: https://x.com/michaelsuttonil/status/2098204180406026482
- Quotes: https://x.com/michaelsuttonil/status/2098089246741397613 (10 Sep 2026)
- Follow-up: https://x.com/michaelsuttonil/status/2098371171498459629
- Write-up: [docs/06-partitioned-defi.md](../docs/06-partitioned-defi.md)

## Post (full text)

> I’d love to hear genuine thoughts about this question (the first part of the text).
>
> My working hypothesis is that global DeFi is not sequential in nature. Reality worldwide happens in parallel. Economic events happen concurrently, information spreads locally and asynchronously, and the global picture forms through aggregation and iterative convergence. There is no single sequence through which the world updates.
>
> That does not make global order in consensus systems any less important. The relative order within sub-series of related events is critical (a double spend being the most minimal/local example). But that should not lead to system designs where ~all state mutations have to go through a constant number of sequential bottlenecks.

He is asking for thoughts on the **first** claim (DeFi is not sequential), not on the ZK ladder in the quoted post.

## Quoted context (10 Sep 2026)

https://x.com/michaelsuttonil/status/2098089246741397613

> so it’s also about shared state bottlenecks, but in that regard i actually believe systems should push app designers into partitioned/parallel/replicated state designs and not vice versa (ie forcing the sys design towards shared state). hence i think that the point of computational scalability is the more fundamental point.
>
> here too there is a ladder. based zk systems force every event to have L1 DA (at the benefit of L1-degree censorship resistance), while non-based systems allow more scalability but less L1-grade security. obv a lot more to say on this topic.

That reply sat under a reminder that even with Silver/Argent further along than expected, the May 2025 line still holds: ZK is the long-term computational scale leap; SC without ZK is inferior.

## Same-thread follow-up (11 Sep 2026, 11:20 UTC)

https://x.com/michaelsuttonil/status/2098371171498459629

> yes. but the shared state tension is very highly relevant in the vprogs discussion as well. if defi reality truly must go through bottlenecks, then nothing can really bring scalability, not even well designed zk. it’s the missing part of my post which i should really write much more about

## Earlier same thesis

- 13 Jul 2026 — Argent: partitioned-state DeFi with atomic composability on L1 UTXO rails. https://x.com/michaelsuttonil/status/2076725908245827780
- 14 Jul 2026 — replica swap ratios; gap bounded as a function of time; for N replicas, converge in log N rounds. Shared-state hotspots are the bad design for global DeFi. https://x.com/michaelsuttonil/status/2076836857829036405

## Replies worth keeping

- Kurchack: chess and checkers on Kaspa don’t share a global sequence; a third app can read both scoreboards.
- NikolaGalilei: can covenants shard state like a chess mutex, and update multiple states inside one tx?
- Sutton (other thread, 11 Sep): he was arguing about physical/financial reality, not the marketing label “DeFi”; practical system-design implications are a separate question.

## Take for this notebook

DAGKnight still orders *related* sub-series (double spend, replica-merge events) at real adversarial latency. It does not justify a single global AMM / nonce / sequencer. A TN10 contention bench (one shared vault vs N replicas) can test the hypothesis before the DK HF. Do not add a global DeFi sequencer opcode to that fork.
