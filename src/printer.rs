use crate::cli::{Cli, ReportingMode};
use crate::math::combinations;
use crate::types::{CompFile, FlattenedMatches};

use std::path::PathBuf;

pub fn now_comparing(args: &Cli, f1: &CompFile, f2: &CompFile) {
    if args.verbose && args.reporting_mode == ReportingMode::Text {
        if f1.file == f2.file {
            eprint!(
                "\rNow comparing {:?} ({:>4}/{:>4})",
                &f1.file,
                f1.start,
                f1.lines.len()
            );
        } else {
            eprint!(
                "\rNow comparing {:?} and {:?} ({:>4}/{:>4})",
                &f1.file,
                &f2.file,
                f1.start,
                f1.lines.len()
            );
        }
    }
}

pub fn done_comparison(args: &Cli, nth: usize) {
    if args.verbose && args.reporting_mode == ReportingMode::Text {
        let total = combinations(args.files.len(), 2) + args.files.len();
        eprintln!("...done {nth}/{total}");
    }
}

pub fn skip_comparison(args: &Cli, f1: &PathBuf, f2: &PathBuf) {
    if args.verbose && args.reporting_mode == ReportingMode::Text {
        if f1 == f2 {
            eprintln!("Unable to open {f1:?} for reading");
        } else {
            eprintln!("Unable to open {f1:?} and {f2:?} for reading");
        }
    }
}

pub fn matches(args: &Cli, matches: &FlattenedMatches) {
    match args.reporting_mode {
        ReportingMode::Json => {
            println!("{}", matches.json());
        }
        ReportingMode::Text => {
            println!("{matches}");
        }
    }
}

pub fn conclusion(args: &Cli, matches: &FlattenedMatches) {
    if args.reporting_mode == ReportingMode::Text && args.verbose {
        eprintln!(
            "A total of {} unique match(es) were found in the {} file(s).",
            matches.unique_matches(),
            args.files.len()
        );
    }
}