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

use rand::Rng;
use rand::rngs::ThreadRng;

pub struct CardGenerator {
    cards: Vec<u32>,
    card_quantities: Option<Vec<usize>>,
    remaining_cards: usize,
    rng: ThreadRng,
}

impl CardGenerator {
    pub fn new(cards: Vec<u32>, dups: usize) -> CardGenerator {
        let remaining_cards = cards.len();

        let quants = if dups > 0 {
            Some(vec![dups; remaining_cards])
        } else {None};

        let card_gen = CardGenerator {
            cards: cards,
            card_quantities: quants,
            remaining_cards: remaining_cards,
            rng: rand::thread_rng(),
        };

        return card_gen;
    }

    // Filter out any duplicates in the given cards Vec and return a CardGenerator
    pub fn new_dedup(cards: Vec<u32>, dups: usize) -> CardGenerator {
        let mut cards_mut = cards.to_vec();
        cards_mut.sort_unstable();
        cards_mut.dedup();

        return CardGenerator::new(cards_mut, dups);
    }

    // Generate a Vec<u32> of length n
    pub fn generate(&mut self, n: usize) -> Vec<u32> {
        let mut generated = Vec::new();

        for _ in 0..n {
            let rand_num: usize = self.rng.gen_range(0..self.remaining_cards);
            generated.push(self.cards[rand_num]);

            if self.card_quantities.is_some() {
                let new_val = self.card_quantities.as_mut().unwrap().remove(rand_num) - 1;
                self.card_quantities.as_mut().unwrap().insert(rand_num, new_val);

                if new_val < 1 {
                    self.remaining_cards -=  1;
                    self.cards.remove(rand_num);
                    self.card_quantities.as_mut().unwrap().remove(rand_num);
                }
            }
        };

        return generated;
    }
}

#[cfg(test)]
mod tests;
