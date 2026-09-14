//! Property test for DAGKnight selected-parent independence of parent order.
//!
//! Paste at the end of `consensus/src/processes/dagknight/protocol.rs`
//! `mod tests` (same Memory* stores + `DagknightExecutor` as
//! `test_duplicate_parents_*`).
//!
//! Paper Alg. 2 does not sort parents. #1104 uses `sort_unstable()` on
//! agreement groups. If that is protocol-equivalent, this passes. If it
//! masks a bug, this fails. Do not merge #1104 without it.

use rand::{SeedableRng, rngs::SmallRng, seq::SliceRandom};

/// Four independent children of genesis. Selected parent of the tip set
/// must not move when the parent slice is shuffled.
#[test]
fn dagknight_selected_parent_invariant_under_parent_permutation() {
    let genesis_hash = Hash::from_u64_word(1);
    let t1 = Hash::from_u64_word(2);
    let t2 = Hash::from_u64_word(3);
    let t3 = Hash::from_u64_word(4);
    let t4 = Hash::from_u64_word(5);

    let mut reachability = MemoryReachabilityStore::new();
    let mut relations = MemoryRelationsStore::new();
    let mut builder = DagBuilder::new(&mut reachability, &mut relations);
    builder.init();
    builder.add_block(DagBlock::new(genesis_hash, vec![ORIGIN]));
    for h in [t1, t2, t3, t4] {
        builder.add_block(DagBlock::new(h, vec![genesis_hash]));
    }

    let headers_store = Arc::new(MemoryHeaderStore::new());
    let mut genesis_header = Header::from_precomputed_hash(genesis_hash, vec![]);
    genesis_header.bits = 0x207fffff;
    headers_store.insert(Arc::new(genesis_header));
    for (i, h) in [t1, t2, t3, t4].into_iter().enumerate() {
        let mut header = Header::from_precomputed_hash(h, vec![genesis_hash]);
        header.bits = 0x207fffff;
        header.daa_score = 1;
        header.blue_score = 1;
        header.timestamp = i as u64 + 1;
        headers_store.insert(Arc::new(header));
    }

    let dk_map = RefCell::new(HashMap::new());
    let dagknight_store = Arc::new(MemoryDagknightStore::new(dk_map));
    let dk_executor = DagknightExecutor {
        genesis_hash,
        dagknight_store,
        headers_store,
        reachability_service: MTReachabilityService::new(Arc::new(RwLock::new(reachability))),
        relations_store: Arc::new(RwLock::new(relations)),
        counters: Arc::new(DagknightCounters::new()),
        umc_persistence_store: Arc::new(MemoryUmcCascadeStore::new()),
    };

    let tips = vec![t1, t2, t3, t4];
    let base = dk_executor.dagknight(&tips);
    assert!(
        tips.contains(&base.selected_parent),
        "selected parent {:?} is not a direct parent of {tips:?}",
        base.selected_parent
    );

    for seed in 0u64..32 {
        let mut p = tips.clone();
        p.shuffle(&mut SmallRng::seed_from_u64(seed));
        let got = dk_executor.dagknight(&p);
        assert_eq!(
            base.selected_parent, got.selected_parent,
            "selected parent moved under permutation seed {seed}: base={:?} got={:?} order={p:?}",
            base.selected_parent, got.selected_parent
        );
    }
}
