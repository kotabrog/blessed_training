#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn ad_hoc_test(weight: usize) {
    #[cfg(feature = "dhat-ad-hoc")]
    dhat::ad_hoc_event(weight);
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    #[cfg(feature = "dhat-ad-hoc")]
    let _profiler = dhat::Profiler::new_ad_hoc();

    println!("Hello, world!");

    let str = String::from("Hello, world!");
    println!("{}", str);

    {
        let str = String::from("Hello, world!");
        println!("{}", str);
    }

    for _ in 0..10 {
        ad_hoc_test(100);
    }

    for _ in 0..5 {
        ad_hoc_test(4)
    }

    let str = String::from("Hello, world!");
    println!("{}", str);
}
