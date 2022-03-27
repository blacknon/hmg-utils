// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

// TODO: ハイフンと伸ばし棒の関係性についても考慮にいれる

use std::collections::HashSet;

mod jpn;
use self::jpn::get_data_kana;

mod width;
use self::width::get_data_width;

use super::common::SplitChar;

#[derive(Clone, Copy)]
pub enum SplitType {
    Kana,
    HalfFullWidth,
}

///
pub fn get_jp_splitchar(split_type: SplitType, chars: Vec<SplitChar>) -> Vec<SplitChar> {
    //
    let mut result = vec![];

    for mut ch in chars {
        // if is_escape, contiune loop.
        if ch.is_escape {
            result.push(ch);
            continue;
        }

        //
        let mut new_ch = vec![];

        // loop check at char
        for c in &ch.char {
            let mut checked_ch = check_data(split_type, c);
            new_ch.append(&mut checked_ch);
        }

        // unique
        let uniq_data: HashSet<String> = new_ch.into_iter().collect();
        ch.char = uniq_data.into_iter().collect();

        result.push(ch);
    }

    return result;
}

///
fn check_data(split_type: SplitType, c: &str) -> Vec<String> {
    let mut result = vec![];

    let mut data: Vec<Vec<&str>> = vec![];
    match split_type {
        SplitType::Kana => {
            data = get_data_kana();
        }
        SplitType::HalfFullWidth => {
            data = get_data_width();
        }
    }

    // loop check at kana
    for kana in data {
        if kana.contains(&c) {
            for k in kana {
                result.push(k.to_string());
            }
            break;
        }
    }

    if result.len() == 0 {
        result.push(c.to_string());
    }

    return result;
}
