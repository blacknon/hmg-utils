// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

// TODO: sjisやeuc-jpなど、複数の文字コードでgrepできるようにする
//       【参考】
//         - https://github.com/lifthrasiir/rust-encoding

// macro crate
#[macro_use]
extern crate clap;
extern crate bytes;
extern crate crossbeam;
extern crate grep;
extern crate grep_matcher;
extern crate grep_regex;
extern crate grep_searcher;
extern crate ignore;
extern crate itertools;
extern crate kana;
extern crate lazy_static;
extern crate memchr;
extern crate rayon;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate termcolor;

// modules
use clap::{App, AppSettings, Arg};
use std::env::args;
use std::path::PathBuf;
use std::str;

// local modules
mod greplib;
mod hmglib;

use greplib::scan;
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
        // -- OPTIONS --
        // Approximate matching settings:
        // -p, --approximate
        // -b, --best-match
        // -D, --delete-cost=NUM     set cost of missing characters
        // -I, --insert-cost=NUM     set cost of extra characters
        // -S, --substitute-cost=NUM set cost of wrong characters
        // -E, --max-errors=NUM     select records that have at most NUM errors
        // -#			    select records that have at most # errors (# is a
        // 　			     digit between 0 and 9)
        //
        // Pattern selection and interpretation:
        // -e, --regexp // gnu grepと同じように、要素単位のオプションにするか？
        // -f, --file
        // -i, --ignore-case
        .arg(Arg::with_name("ignore_case").short("i"))
        // -k, --literal
        // -w, --word-regexp
        // -z, --null-data
        // -W, --with-word
        // -u, --multiunicode
        //   ... 他の文字コードで検索できるようにする
        // -H, --disable-homoglyphs
        //   ... ホモグリフ変換を無効化
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
        // Approximate matching settings:
        //
        // Miscellaneous:
        // -d, --delimiter
        // -s, --no-messages
        // -v, --invert-match
        // -V, --version
        //
        // Output control:
        // -m, --max-count=NUM
        // .arg(Arg::with_name("max_count").short("m").default_value("0"))
        // -n, --line-number
        .arg(Arg::with_name("line_number").short("n"))
        //     --line-buffered
        //     --with-filename
        //     --no-filename
        //     --label=LABEL
        // -o, --only-matching
        .arg(Arg::with_name("only_matching").short("o"))
        // -q, --quiet, --silent
        //     --binary-files=TYPE
        // -a, --text
        // -r, --recursive
        // -R, --dereference-recursive
        //     --include=GLOB
        //     --exclude=GLOB
        //     --exclude-from=FILE
        //     --exclude-dir=GLOB
        //     --files-without-match
        //     --files-with-matches
        // -c, --count
        // -T, --initial-tab
        // -Z, --null
        //
        // Context control:
        // -B, --before-context=NUM
        .arg(
            Arg::with_name("before_context")
                .short("B")
                .takes_value(true)
                .default_value("0"),
        )
        // -A, --after-context=NUM
        .arg(
            Arg::with_name("after_context")
                .short("A")
                .takes_value(true)
                .default_value("0"),
        )
        // -C, --context=NUM
        // -NUM
        // -P, --parallel
        // --color
        // --colour
        // .arg(
        //     Arg::with_name("color")
        //         .help("use markers to highlight the matching strings")
        //         .long("color")
        //         .long("colour")
        //         .takes_value(true)
        //         .default_value("auto")
        //         .possible_values(&["always", "auto", "never"]),
        // )
        // -U, --binary
        // -- PATTERNS --
        .arg(
            Arg::with_name("PATTERNS")
                .allow_hyphen_values(true)
                .multiple(false)
                .required(true),
        )
        // -- PATH --
        .arg(
            Arg::with_name("PATH")
                .allow_hyphen_values(true)
                .multiple(true)
                .required(true),
        )
}

///
fn main() {
    // Get command args matches
    let matches = build_app().get_matches();

    // Get arg `PATTERNS`
    let text = matches.value_of("PATTERNS");
    let paths = matches.values_of("PATH");
    let path_list = PathBuf::from(paths.unwrap().next().unwrap());

    // Get Homoglyphs options
    let is_japanese_kana = matches.is_present("japanese_kana");
    let is_cjk_width = matches.is_present("cjk_width");

    // Get Grep options
    let ignore_case = matches.is_present("ignore_case");
    // let mut max_count = value_t!(matches, "max_count", u64).unwrap_or_else(|e| e.exit());
    let only_matching = matches.is_present("only_matching");
    let line_number = matches.is_present("line_number");
    let before_context = value_t!(matches, "before_context", usize).unwrap_or_else(|e| e.exit());
    let after_context = value_t!(matches, "after_context", usize).unwrap_or_else(|e| e.exit());

    // Get Homoglyphs object
    let pattern = Homoglyphs::new()
        .with_cjk_width(is_cjk_width)
        .with_japanese_kana(is_japanese_kana)
        .get_pattern(text.unwrap());

    // Get regex pattern
    let regex_pattern = hmglib::generate_pattern_regex(pattern);

    // Get grep
    let config = greplib::Config::new()
        .after_context(after_context)
        .before_context(before_context)
        .only_matching(only_matching)
        .case_insensitive(ignore_case)
        .line_number(line_number)
        .build();

    greplib::scan(config, &regex_pattern, vec![path_list]);
}
