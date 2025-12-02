use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use gift_shop::GiftShop;
use secret_entrance::SecretEntrance;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-secret-entrance/input.txt",
        SecretEntrance,
        "Combined"
    ),
    (
        day_002,
        "../day-002-gift-shop/input.txt",
        GiftShop,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
