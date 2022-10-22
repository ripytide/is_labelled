use std::{collections::HashMap, path::Path};

use colored::Colorize;
use walkdir::WalkDir;

type LabelCounts = HashMap<String, LabelCount>;
type LabelCount = HashMap<String, u64>;

const TESTED: &str = "#[tested]";
const UNTESTED: &str = "#[untested]";
const PARENT_TESTED: &str = "#[parent_tested]";
const TRIVIAL: &str = "#[trivial]";
const NOT_A_FN: &str = "#[not_a_fn]";

fn main() {
	let args: Vec<String> = std::env::args().collect();

	let be_loud = args.len() == 2;

	let label_counts = get_label_counts(args[1].clone(), be_loud);

	if be_loud {
		if label_counts.is_empty() {
			println!(
				"{}",
				"All function labelled! \u{1f680}\u{1f680}\u{1f680}"
					.bold()
			);
		} else {
			println!( "{}", "Not all function labelled! \u{1f61f}\u{1f61f}\u{1f61f}".bold());
		}
	} else {
		print_label_counts(&label_counts);
	}
}

fn print_label_counts(label_counts: &LabelCounts) {
	for (filename, label_count) in label_counts {
		if !label_count.is_empty() {
			println!("{}", filename.yellow());
			for (label, x) in label_count {
				println!(
					"label: {} total: {}",
					format_label(label),
					x.to_string().red()
				)
			}

			println!("");
		}
	}
}

fn format_label(label: &String) -> String {
	let str = if label.contains(TESTED) {
		TESTED.bright_green()
	} else if label.contains(PARENT_TESTED) {
		PARENT_TESTED.green()
	} else if label.contains(UNTESTED) {
		UNTESTED.red()
	} else if label.contains(TRIVIAL) {
		TRIVIAL.blue()
	} else if label.contains(NOT_A_FN) {
		NOT_A_FN.purple()
	} else {
		"".white()
	};

	return str.to_string();
}

fn get_label_counts(filename: String, be_loud: bool) -> LabelCounts {
	let mut label_counts = LabelCounts::new();

	for input_filename in WalkDir::new(filename)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file())
	{
		let label_count =
			get_label_count(input_filename.path(), be_loud).unwrap();
		label_counts.insert(
			input_filename.path().display().to_string(),
			label_count,
		);
	}

	return label_counts;
}

fn get_label_count(
	filename: &Path,
	be_loud: bool,
) -> Result<LabelCount, std::io::Error> {
	let file_string = std::fs::read_to_string(&filename)?;
	let lines: Vec<&str> = file_string.split("\n").collect();

	let mut label_count = LabelCount::new();

	for (index, line) in lines
		.iter()
		.take_while(|line| !line.contains("cfg(test)"))
		.enumerate()
	{
		if line.contains("fn ") {
			if index == 0
				|| !contains_valid_label(
					lines[index - 1],
					&mut label_count,
				) {
				if be_loud {
					println!(
						"Unlabeled function on line {} in file: {}",
						index.to_string().red(),
						filename.display().to_string().bold()
					);
				}
			}
		}
	}

	return Ok(label_count);
}

fn contains_valid_label(
	string: &str,
	label_count: &mut LabelCount,
) -> bool {
	if string.contains(TESTED) {
		label_count
			.entry(TESTED.to_string())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	} else if string.contains(PARENT_TESTED) {
		label_count
			.entry(PARENT_TESTED.to_string())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	} else if string.contains(UNTESTED) {
		label_count
			.entry(UNTESTED.to_string())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	} else if string.contains(TRIVIAL) {
		label_count
			.entry(TRIVIAL.to_string())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	} else if string.contains(NOT_A_FN) {
		label_count
			.entry(NOT_A_FN.to_string())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	} else {
		return false;
	}

	return true;
}

//tool checks that all functions not in a test module have one and only one of the accepted
//labels [tested, parent_tested, untested]
//also checks to make sure no labels are used inside test mods
//can also tell the counts of different labels per file
