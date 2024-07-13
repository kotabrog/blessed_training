fn ad_hoc_test(weight: usize) {
    dhat::ad_hoc_event(weight);
}

#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().ad_hoc().testing().build();

    for _ in 0..10 {
        ad_hoc_test(100);
    }

    for _ in 0..5 {
        ad_hoc_test(4)
    }

    let stats = dhat::AdHocStats::get();

    dhat::assert_eq!(stats.total_events, 15);
    dhat::assert_eq!(stats.total_units, 1020);
}
