// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

// macro crate
#[macro_use]
extern crate clap;
extern crate itertools;
extern crate kana;
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate serde_json;

// modules
use clap::{App, AppSettings, Arg};
use std::env::args;

// local modules
mod hmglib;
use hmglib::Homoglyphs;

/// Parse args and options function.
fn build_app() -> clap::App<'static, 'static> {
    // get own name
    let _program = args()
        .nth(0)
        .and_then(|s| {
            std::path::PathBuf::from(s)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap();

    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::AllowLeadingHyphen)
        // -- PATTERNS --
        .arg(
            Arg::with_name("PATTERNS")
                .allow_hyphen_values(true)
                .multiple(false)
                .required(true),
        )
        // -- OPTIONS --
        // Pattern selection and interpretation:
        // -f, --file
        .arg(
            Arg::with_name("file")
                .help("Search by leet conversion of PATTERNS.")
                .short("f")
                .long("file"),
        )
        // -e, --regex
        .arg(
            Arg::with_name("regex")
                .help("Generate in regular expression format.")
                .short("e")
                .long("regex"),
        )
        // -k, --literal
        .arg(
            Arg::with_name("literal")
                .help("Treat as a literal.")
                .short("k")
                .long("literal"),
        )
        // -i, --ignore-case
        .arg(
            Arg::with_name("ignore_case")
                .help("Treat case-insensitive")
                .short("i")
                .long("ignore-case"),
        )
        // leet includes converted keywords in search
        //   [-L, --leet]
        // .arg(
        //     Arg::with_name("leet")
        //         .help("Convert a string to leet")
        //         .short("L")
        //         .long("leet"),
        // )
        // -j, --japanese-kana
        .arg(
            Arg::with_name("japanese_kana")
                .help("Not distinguish between Hiragana and Katakana.")
                .short("j"),
        )
        // -W, --cjk-width
        .arg(
            Arg::with_name("cjk_width")
                .help("Not distinguish between half-width and full-width at CJK.")
                .short("W")
                .long("cjk-width"),
        )
}

///
fn main() {
    // Get command args matches
    let matches = build_app().get_matches();

    // Get args
    let text = matches.value_of("PATTERNS");
    let is_regex = matches.is_present("regex");
    let is_literal = matches.is_present("literal");
    let is_ignore = matches.is_present("ignore_case");

    let is_japanese_kana = matches.is_present("japanese_kana");
    let is_cjk_width = matches.is_present("cjk_width");

    // Get pattern from Homoglyphs object.
    let pattern = Homoglyphs::new()
        .is_ignore_case(is_ignore)
        .is_literal(is_literal)
        .with_japanese_kana(is_japanese_kana)
        .with_cjk_width(is_cjk_width)
        .get_pattern(text.unwrap());

    // if add -e flag, output regex pattern.
    if is_regex {
        let regex_pattern = hmglib::generate_pattern_regex(pattern.clone());
        println!("{}", regex_pattern);
        return;
    }

    let data = hmglib::generate_pattern_list(pattern);
    for d in data {
        println!("{}", d);
    }
}
