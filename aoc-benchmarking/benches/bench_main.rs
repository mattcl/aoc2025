use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use cafeteria::Cafeteria;
use gift_shop::GiftShop;
use lobby::Lobby;
use printing_department::PrintingDepartment;
use secret_entrance::SecretEntrance;
use trash_compactor::TrashCompactor;
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
    (
        day_003,
        "../day-003-lobby/input.txt",
        Lobby,
        "Combined"
    ),
    (
        day_004,
        "../day-004-printing-department/input.txt",
        PrintingDepartment,
        "Combined"
    ),
    (
        day_005,
        "../day-005-cafeteria/input.txt",
        Cafeteria,
        "Combined"
    ),
    (
        day_006,
        "../day-006-trash-compactor/input.txt",
        TrashCompactor,
        "Combined"
    ),
    // bench_marker
}
