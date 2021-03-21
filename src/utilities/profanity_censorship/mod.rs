//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

use std::{
    collections::{
        HashMap
    },
    lazy::{
        SyncLazy
    }
};

static CHARACTER_ALIASES: SyncLazy<HashMap<char, char>> = SyncLazy::new(|| {
    let mut hashmap = HashMap::<char, char>::new();
    const CASE_DIFFERENCE: u8 = b'a' - b'A';

    for character in b'A'..=b'Z' {
        hashmap.insert(character as char, (character + CASE_DIFFERENCE) as char);
    }

    // aliases for the letter A.
    ['4', '@', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'à', 'á', 'â', 'ã', 'ä', 'å', 'α', 'Α']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'a');
        });

    // aliases for the letter B.
    ['ß', 'Β', '฿']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'b');
        });

    // aliases for the letter C.
    ['¢', 'ç', 'Ç', '©']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'c');
        });

    // aliases for the letter D.
    ['Ð', '₫']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'd');
        });

    // aliases for the letter E.
    ['3', '£', '€', 'È', 'É', 'Ê', 'Ë', 'è', 'é', 'ê', 'ë', 'ε', 'Ε', 'Ξ', 'Σ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'e');
        });

    hashmap
});
