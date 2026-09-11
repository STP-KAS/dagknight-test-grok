//! Append to kaspanet/rusty-kaspa `dagknight` branch:
//! `consensus/src/pipeline/virtual_processor/tests.rs`
//!
//! Extra imports needed:
//!   crate::model::stores::ghostdag::GhostdagStoreReader
//!   kaspa_consensus_core::config::params::ForkActivation

/// Pipeline e2e with DAGKnight activation on: two merges so rank/UMC actually run.
/// Structural invariants only (selected parent is a direct parent; megachain
/// reaches genesis; both coloring and topology stores are populated).
#[tokio::test]
async fn dagknight_pipeline_e2e() {
    let config = ConfigBuilder::new(MAINNET_PARAMS)
        .skip_proof_of_work()
        .edit_consensus_params(|p| {
            p.min_difficulty_window_size = p.difficulty_window_size;
            p.dagknight_activation = ForkActivation::always();
        })
        .build();
    let consensus = TestConsensus::new(&config);
    let handles = consensus.init();
    let genesis = config.genesis.hash;

    //     A (genesis)
    //    / \
    //   B   C
    //    \ / \
    //     D   F
    //     |  /
    //     E /
    //      X
    let b: Hash = 2.into();
    let c: Hash = 3.into();
    let d: Hash = 4.into();
    let e: Hash = 5.into();
    let f: Hash = 6.into();
    let x: Hash = 7.into();
    let blocks: Vec<(Hash, Vec<Hash>)> = vec![
        (b, vec![genesis]),
        (c, vec![genesis]),
        (d, vec![b, c]),
        (e, vec![d]),
        (f, vec![c]),
        (x, vec![e, f]),
    ];

    for (hash, parents) in &blocks {
        let header = consensus.build_header_with_parents(*hash, parents.clone(), true);
        consensus
            .validate_and_insert_block(Block::from_header(header))
            .virtual_state_task
            .await
            .unwrap_or_else(|err| panic!("DK pipeline rejected {hash}: {err}"));
    }

    for (hash, parents) in &blocks {
        let coloring = consensus.coloring_ghostdag_store.get_data(*hash).expect("coloring GD");
        let topology = consensus.topology_ghostdag_store.get_data(*hash).expect("topology GD");
        assert!(
            parents.contains(&coloring.selected_parent),
            "coloring SP {} of {hash} not in {parents:?}",
            coloring.selected_parent
        );
        assert!(
            parents.contains(&topology.selected_parent),
            "topology SP {} of {hash} not in {parents:?}",
            topology.selected_parent
        );
        assert!(coloring.blue_score >= 1, "coloring blue score of {hash} is 0");
    }

    let mut cursor = x;
    let mut hops = 0usize;
    while cursor != genesis {
        cursor = consensus.coloring_ghostdag_store.get_data(cursor).unwrap().selected_parent;
        hops += 1;
        assert!(hops <= blocks.len() + 1, "coloring selected-parent walk missed genesis");
    }

    consensus.shutdown(handles);
}
