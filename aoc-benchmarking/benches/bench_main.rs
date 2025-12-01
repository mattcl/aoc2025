use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
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
    // bench_marker
}
