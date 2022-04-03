// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

use std::cmp;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use crossbeam::queue::ArrayQueue;
use termcolor::{Ansi, NoColor, WriteColor};

// ripgrep modules
use grep::matcher::LineTerminator;
use grep::printer::{ColorSpecs, Standard, StandardBuilder};
use grep_regex::{RegexMatcher, RegexMatcherBuilder};
use grep_searcher::{BinaryDetection, MmapChoice, Searcher, SearcherBuilder, Sink, SinkMatch};
use ignore::{DirEntry, Walk, WalkBuilder, WalkParallel};

// const
#[cfg(windows)]
const LINE_ENDING: u8 = b"\r\n";

#[cfg(not(windows))]
const LINE_ENDING: u8 = b'\n';

// enum
#[derive(Clone, Copy)]
pub enum ColorMode {
    None,
    Auto,
    Always,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Auto
    }
}

#[derive(Default, Clone, Copy)]
pub struct Config<'main> {
    /// The number of lines after a match to include.
    /// This is a searcher config item.
    after_context: usize,

    /// The number of lines before a match to include.
    /// This is a searcher config item.
    before_context: usize,

    color: ColorMode,

    /// The binary data detection strategy.
    /// This is a searcher config item.
    binary: bool,

    /// Whether to do automatic transcoding based on a BOM or not.
    /// This is a searcher config item.
    bom_sniffing: bool,

    /// Set the value for the case insensitive (i) flag.
    /// This is a matcher config item.
    case_insensitive: bool,

    /// An encoding that, when present, causes the searcher to transcode all
    /// input from the encoding to UTF-8.
    /// This is a searcher config item.
    encoding: Option<&'main str>,

    /// Only print the specific matches instead of the entire line containing each match.
    /// It behaves the same as `grep -o`.
    /// This is a printer config item.
    pub only_matching: bool,

    /// Set the value for the ignore whitespace (x) flag.
    /// This is a matcher config item.
    ignore_whitespace: bool,

    /// Whether to invert matching.
    /// This is a searcher config item.
    invert_match: bool,

    /// Whether to enable unbounded context or not.
    /// This is a searcher config item.
    passthru: bool,

    /// Whether to count line numbers.
    /// This is a searcher config item.
    line_number: bool,

    /// The memory map strategy.
    /// This is a searcher config item.
    mmap: bool,

    /// Whether to enable matching across multiple lines.
    /// This is a searcher config item.
    multi_line: bool,

    /// Set the maximum amount of matching lines that are printed.
    /// This is a printer config item.
    pub max_matches: Option<u64>,
}

impl<'main> Config<'main> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }

    pub fn after_context(&mut self, num: usize) -> &mut Self {
        self.after_context = num;
        self
    }

    pub fn before_context(&mut self, num: usize) -> &mut Self {
        self.before_context = num;
        self
    }

    pub fn binary(&mut self, yes: bool) -> &mut Self {
        self.binary = yes;
        self
    }

    pub fn bom_sniffing(&mut self, yes: bool) -> &mut Self {
        self.bom_sniffing = yes;
        self
    }

    pub fn color(&mut self, color_mode: ColorMode) -> &mut Self {
        self.color = color_mode;
        self
    }

    pub fn case_insensitive(&mut self, yes: bool) -> &mut Self {
        self.case_insensitive = yes;
        self
    }

    pub fn encoding(&mut self, enc: &'main str) -> &mut Self {
        self.encoding = Some(enc);
        self
    }

    pub fn only_matching(&mut self, yes: bool) -> &mut Self {
        self.only_matching = yes;
        self
    }

    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
        self.ignore_whitespace = yes;
        self
    }

    pub fn invert_match(&mut self, yes: bool) -> &mut Self {
        self.invert_match = yes;
        self
    }

    pub fn passthru(&mut self, yes: bool) -> &mut Self {
        self.passthru = yes;
        self
    }

    pub fn line_number(&mut self, yes: bool) -> &mut Self {
        self.line_number = yes;
        self
    }

    pub fn mmap(&mut self, yes: bool) -> &mut Self {
        self.mmap = yes;
        self
    }

    pub fn multi_line(&mut self, yes: bool) -> &mut Self {
        self.multi_line = yes;
        self
    }

    pub fn max_matches(&mut self, num: u64) -> &mut Self {
        self.max_matches = Some(num);
        self
    }

    pub fn build_matcher(&mut self, pattern: &str) -> Result<RegexMatcher, Box<Error>> {
        Ok(RegexMatcherBuilder::new()
            .case_insensitive(self.case_insensitive)
            .ignore_whitespace(self.ignore_whitespace)
            .build(&pattern)?)
    }

    pub fn build_searcher(&mut self) -> Searcher {
        let mut search_builder = SearcherBuilder::new();

        search_builder
            .after_context(self.after_context)
            .before_context(self.before_context)
            .line_terminator(LineTerminator::byte(LINE_ENDING))
            .line_number(self.line_number)
            .multi_line(self.multi_line);

        if self.binary {
            search_builder.binary_detection(BinaryDetection::quit(0));
        }

        search_builder.build()
    }

    pub fn build_walker(&mut self, path_list: Vec<PathBuf>) -> Result<WalkParallel, Box<Error>> {
        let mut paths = path_list.iter();
        let mut builder = WalkBuilder::new(paths.next().unwrap());
        builder.ignore(true).threads(cmp::min(12, num_cpus::get()));

        // paths.for_each(|p| {
        //     builder.add(p);
        // });

        Ok(builder.build_parallel())
    }
}

pub fn scan(config: Config, pattern: &str, path_list: Vec<PathBuf>) {
    let mut config = config.clone();

    let matcher = config.build_matcher(pattern).unwrap();
    let walker = config.build_walker(path_list);

    // let queue: Arc<ArrayQueue<Option<String>>> = Arc::new(ArrayQueue::new(10));

    walker.unwrap().run(|| {
        // let push_queue = queue.clone();
        let matcher = matcher.clone();
        let mut searcher = config.build_searcher();

        return Box::new(move |entry| {
            if let Ok(entry) = entry {
                let mut printer = StandardBuilder::new()
                    .only_matching(config.only_matching)
                    .max_matches(config.max_matches)
                    .color_specs(ColorSpecs::default_with_color())
                    .build(Ansi::new(vec![]));

                let _ = searcher.search_path(&matcher, entry.path(), printer.sink(&matcher));
                let output = String::from_utf8(printer.into_inner().into_inner()).unwrap();

                // push_queue.push(Some(output));
                print!("{}", output);
            }
            ignore::WalkState::Continue
        });
    });
}
