use std::env;

use aoc_plumbing::Problem;
use cafeteria::Cafeteria;
use factory::Factory;
use gift_shop::GiftShop;
use laboratories::Laboratories;
use lobby::Lobby;
use movie_theater::MovieTheater;
use playground::Playground;
use printing_department::PrintingDepartment;
use reactor::Reactor;
use secret_entrance::SecretEntrance;
use trash_compactor::TrashCompactor;
// import_marker

macro_rules! generate_cli {
    ($(($name:ident, $day:literal)),* $(,)?) => {
        pub fn run() -> anyhow::Result<()> {
            let day: u8 = env::var("AOC_DAY")?.parse()?;
            let input_file = env::var("AOC_INPUT")?;
            let input = std::fs::read_to_string(&input_file)?;
            match day {
                $(
                // $day => serde_json::to_string(&$name::solve(&input)?)?,
                $day => {
                    let sln = $name::solve(&input)?;
                    println!(r#"{{"part_one": {}, "part_two": {}}}"#, sln.part_one, sln.part_two);
                },
                )*
                _ => { println!("\"not implemented\""); }
            }

            Ok(())
        }
    }
}

generate_cli! {
    (SecretEntrance, 1),
    (GiftShop, 2),
    (Lobby, 3),
    (PrintingDepartment, 4),
    (Cafeteria, 5),
    (TrashCompactor, 6),
    (Laboratories, 7),
    (Playground, 8),
    (MovieTheater, 9),
    (Factory, 10),
    (Reactor, 11),
    // command_marker
}
