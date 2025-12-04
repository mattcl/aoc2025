use std::env;

use aoc_plumbing::Problem;
use gift_shop::GiftShop;
use lobby::Lobby;
use printing_department::PrintingDepartment;
use secret_entrance::SecretEntrance;
// import_marker

macro_rules! generate_cli {
    ($(($name:ident, $day:literal)),* $(,)?) => {
        pub fn run() -> anyhow::Result<()> {
            let day: u8 = env::var("AOC_DAY")?.parse()?;
            let input_file = env::var("AOC_INPUT")?;
            let input = std::fs::read_to_string(&input_file)?;
            let out = match day {
                $(
                $day => serde_json::to_string(&$name::solve(&input)?)?,
                )*
                _ => "\"not implemented\"".into(),
            };

            println!("{}", out);

            Ok(())
        }
    }
}

generate_cli! {
    (SecretEntrance, 1),
    (GiftShop, 2),
    (Lobby, 3),
    (PrintingDepartment, 4),
    // command_marker
}
