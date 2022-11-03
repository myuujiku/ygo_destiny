/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::*;

use crate::logic::utils::cache::CACHE;
use crate::logic::utils::http;

fn init_cg() -> CardGenerator {
    CardGenerator::new(vec![1], 3)
}

#[test]
fn test_generate() {
    assert_eq!(vec![1; 1], init_cg().generate(1));
    assert_eq!(vec![1; 2], init_cg().generate(2));
    assert_eq!(vec![1; 3], init_cg().generate(3));
}

#[test]
fn test_quants() {
    let mut cg = init_cg();
    cg.generate(1);
    assert_eq!(&vec![2], cg.card_quantities.as_ref().unwrap());

    cg.generate(1);
    assert_eq!(&vec![1], cg.card_quantities.as_ref().unwrap());

    cg.generate(1);
    assert_eq!(&Vec::<usize>::new(), cg.card_quantities.as_ref().unwrap());
}

#[test]
fn test_result_len() {
    let mut cg = CardGenerator::new(vec![1; 9], 9);
    for i in 0..=9 {
        assert_eq!(cg.generate(i).len(), i);
    }
}

#[test]
fn test_new_dedup() {
    let cg_dd = CardGenerator::new_dedup(vec![1, 2, 1, 2, 3, 3], 3);
    assert_eq!(cg_dd.remaining_cards, 3);
}

#[test]
fn test_real() {
    http::load_local_data();
    let cards = CACHE.lock().unwrap().get_cards_from_sets(vec!["Spell Ruler".to_string()]);
    assert_eq!(104, cards.len());
    let mut cg = CardGenerator::new(cards.to_vec(), 3);
    assert_eq!(true, cards.contains(&cg.generate(1)[0]));
}
