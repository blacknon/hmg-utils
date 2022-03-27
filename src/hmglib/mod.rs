// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

// TODO: かな変換処理を入れる(読みとか…)
// TODO: 合字対応(機種依存文字への対応(昭和=>㍼)など)とかCJK互換用文字とか…
//       - http://www.asahi-net.or.jp/~ax2s-kmtn/ref/unicode/ufb00.html
//       - https://ja.wikipedia.org/wiki/CJK%E4%BA%92%E6%8F%9B%E7%94%A8%E6%96%87%E5%AD%97
//       - http://www.asahi-net.or.jp/~ax2s-kmtn/ref/unicode/u3300.html
// TODO: 記号系(含む機種依存文字)をどうするか考える（同じような意味を持つ記号の対応を取り入れるか否か）
//       - ←→ ... ⇐⇒
// TODO: 日本語に限らず、CJKの文字コードや記号の半角にも対応させる
//       (色々面倒なので、ライブラリ(unicode-jp-rs)に切り替えて、そちらに手を入れて対応させる？)
// TODO: キーワード単位でパース処理をする仕組みを実装する(それを実装しないと↑の処理は実装難しそう)

use std::collections::HashSet;

// local module
mod cartesian;
mod cjk;
mod common;
mod homoglyph;

// use self
use self::common::SplitChar;

/// Struct for homograph conversion.
#[derive(Default)]
pub struct Homoglyphs {
    // is_literal is enabled,
    is_literal: bool,

    //
    is_ignore_case: bool,

    // with_leet is enabled, leet conversion is also performed.
    with_leet: bool,

    //
    with_japanese_kana: bool,

    //
    with_cjk_width: bool,
}

/// Impl for homograph conversion
impl Homoglyphs {
    //
    pub fn new() -> Self {
        let hmg = Self::default();
        return hmg;
    }

    //
    pub fn is_literal(&mut self, yes: bool) -> &mut Self {
        self.is_literal = yes;
        return self;
    }

    //
    pub fn is_ignore_case(&mut self, yes: bool) -> &mut Self {
        self.is_ignore_case = yes;
        return self;
    }

    //
    pub fn with_leet(&mut self, yes: bool) -> &mut Self {
        self.with_leet = yes;
        return self;
    }

    //
    pub fn with_japanese_kana(&mut self, yes: bool) -> &mut Self {
        self.with_japanese_kana = yes;
        return self;
    }

    //
    pub fn with_cjk_width(&mut self, yes: bool) -> &mut Self {
        self.with_cjk_width = yes;
        return self;
    }

    //
    pub fn get_pattern(&self, plane: &str) -> Vec<Vec<String>> {
        let mut text = plane.to_string();

        //
        let mut result = vec![];

        // regex to escape.
        if !self.is_literal {
            text = regex::escape(&text);
        }

        // split char
        let mut chars: Vec<SplitChar> = split_regex2schar(&text);

        // check japanese kana
        if self.with_japanese_kana {
            chars = cjk::get_jp_splitchar(cjk::SplitType::Kana, chars);
        }

        // check japanese half-widthd
        if self.with_cjk_width {
            chars = cjk::get_jp_splitchar(cjk::SplitType::HalfFullWidth, chars);
        }

        // check japanese kana(2nd stage).
        if self.with_japanese_kana {
            chars = cjk::get_jp_splitchar(cjk::SplitType::Kana, chars);
        }

        // get Homoglyphs data.
        for c in chars {
            if c.is_escape {
                let mut push_data: Vec<String> = vec![];
                for ch in c.char {
                    let mut char_data = vec![ch.escape_default().to_string()];
                    push_data.append(&mut char_data);
                }
                result.push(push_data)
            } else {
                //
                let mut push_data: Vec<String> = vec![];

                for ch in c.char {
                    let mut split_data = contains_hmg_text(ch.clone());
                    push_data.append(&mut split_data);
                }
                let uniq_data: HashSet<String> = push_data.into_iter().collect();

                push_data = uniq_data.into_iter().collect();

                //
                result.push(push_data);
            }
        }

        return result;
    }
}

///
pub fn generate_pattern_regex(pattern: Vec<Vec<String>>) -> String {
    let mut regex_text = vec![];
    for p in pattern {
        if p.len() > 1 {
            regex_text.push("(".to_string());
        }

        let t = p.join("|");
        regex_text.push(t);

        if p.len() > 1 {
            regex_text.push(")".to_string());
        }
    }

    return regex_text.join("");
}

///
pub fn generate_pattern_list(pattern: Vec<Vec<String>>) -> Vec<String> {
    // TODO: 並列数を指定して処理を実行させる
    let result = cartesian::get_cartesian_product(&pattern);

    return result;
}

///
fn contains_hmg_text(pchar: String) -> Vec<String> {
    // result
    let mut result: Vec<String> = vec![];

    // get homoglyphs data
    let lines = homoglyph::get_homoglyphs();

    // for loop check
    for l in &lines {
        let mut data = l.data.clone();
        if data.contains(&pchar) {
            result.append(&mut data);
            break;
        }
    }

    // check pchar to char.
    let chars: Vec<char> = pchar.chars().collect();
    if chars.len() > 1 {
        //
        let mut chars_data: Vec<Vec<String>> = vec![];

        // for chars check...
        for c in chars {
            let mut is_hit = false;

            // for loop check
            for l in &lines {
                let data = l.data.clone();
                if data.contains(&c.to_string()) {
                    chars_data.push(data);
                    is_hit = true;
                    break;
                }
            }

            if !is_hit {
                chars_data.push(vec![c.to_string()]);
            }
        }

        let mut chars_product = cartesian::get_cartesian_product(&chars_data);
        result.append(&mut chars_product);
    }

    // if result is none, return pchar.
    if result.len() == 0 {
        result = vec![pchar];
    }

    return result;
}

///
fn split_regex2schar(regex_text: &str) -> Vec<SplitChar> {
    let mut result = vec![];

    // create char
    let chars: Vec<char> = regex_text.chars().collect();

    let mut is_escape = false;
    for c in chars {
        // check is_escape
        if c == '\\' {
            is_escape = true;
            continue;
        }

        let sc = SplitChar {
            char: vec![c.to_string()],
            is_escape: is_escape.clone(),
        };

        result.push(sc);

        is_escape = false;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_plane_contains_hmg_text() {
        let data = contains_hmg_text("女".to_string());
        println!("{:?}", data);
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn tests_dakuon1_contains_hmg_text() {
        let dakuten1 = contains_hmg_text("は゛".to_string());
        let dakuten2 = contains_hmg_text("ハ゜".to_string());
        println!("{:?}", dakuten1);
        println!("{:?}", dakuten2);
        assert_eq!(dakuten1.len(), 2);
        assert_eq!(dakuten2.len(), 6);
    }
}
