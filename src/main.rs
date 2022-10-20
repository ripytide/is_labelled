use colored::Colorize;
use std::path::Path;

use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    for input_filename in WalkDir::new(&args[1])
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        process_file(input_filename.path())?;
    }

    return Ok(());
}

fn process_file(filename: &Path) -> Result<(), std::io::Error> {
    let file_string = std::fs::read_to_string(&filename)?;

    let lines: Vec<&str> = file_string.split("\n").collect();

    for (index, line) in lines
        .iter()
        .take_while(|line| !line.contains("cfg(test)"))
        .enumerate()
    {
        if line.contains("fn") {
            if index == 0 || !contains_valid_label(lines[index - 1]) {
                println!(
                    "Unlabeled function on line {} in file: {}",
                    index.to_string().red(),
                    filename.display().to_string().bold()
                );
            }
        }
    }

    return Ok(());
}

fn contains_valid_label(string: &str) -> bool {
    if string.contains("tested") || string.contains("parent_tested") || string.contains("untested")
    {
        return true;
    } else {
        return false;
    }
}

//tool checks that all functions not in a test module have one and only one of the accepted
//labels [tested, parent_tested, untested]
//also checks to make sure no labels are used inside test mods
//can also tell the counts of different labels per file
