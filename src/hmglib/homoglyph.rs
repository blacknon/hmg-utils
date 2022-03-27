// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

/// Data ofthe characters of the corresponding homograph.
/// Check `src/hmglib/README.md` for how to generate this file.
const DATA_JSON_HOMOGLYPHS: &str = include_str!("files/homoglyph.json");

#[derive(Serialize, Deserialize)]
pub struct HomoglyphData {
    pub data: Vec<String>,
}

pub fn get_homoglyphs() -> Vec<HomoglyphData> {
    // json to Vec in struct
    let data: Vec<HomoglyphData> = serde_json::from_str(DATA_JSON_HOMOGLYPHS).unwrap();

    return data;
}
