use std::{
    fs::{read_dir, read_to_string},
    io,
    path::PathBuf,
};

use crate::solvable::Registry;

mod solutions;
mod solvable;

fn main() -> io::Result<()> {
    let source_dir = "../inputs";
    let mut paths: Vec<PathBuf> = read_dir(source_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .collect();
    paths.sort();

    let inputs = paths
        .into_iter()
        .filter_map(|path| read_to_string(path).ok())
        .collect::<Vec<_>>();

    let registry = Registry::new();

    let solutions = registry.solve(&inputs);

    solutions
        .iter()
        .enumerate()
        .for_each(|(i, (first, second))| {
            println!("Day {}:", i + 1);
            println!("Task 1 -> {first}");
            println!("Task 2 -> {second}");
            println!("--------------------------------------");
        });

    Ok(())
}
