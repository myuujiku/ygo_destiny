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

//! # Examples
//!
//! ## CardGenerator
//!
//! A `CardGenerator` can handle any `u32` value. In a real environment it only makes sense to
//! use actual card ids as values.
//! To get all cards from one or more sets see
//! [`Cache::get_cards_from_sets`][`crate::logic::utils::cache::Cache::get_cards_from_sets`].
//!
//! ```rust
//! use ygo_destiny::logic::utils::card_gen::CardGenerator;
//!
//! // Define the card pool
//! let card_pool: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
//!
//! // Set how many times each card should be in the card pool
//! let duplicates: u32 = 3;
//!
//! // Create a new card generator
//! let mut cg = CardGenerator::new(card_pool, duplicates);
//!
//! // Generate 5 cards
//! let _cards: Vec<u32> = cg.generate(5);
//!
//! // Generate 4 batches of cards with 2 cards each
//! let batches = 4;
//! let cards_per_batch = 2;
//! let cards: Vec<Vec<u32>> = cg.batch_generate(batches, cards_per_batch);
//!
//! assert_eq!(cards.len(), batches);
//! assert_eq!(cards[0].len(), cards_per_batch);
//! ```
//!
//! To remove an duplicates from the card pool beforehand use `CardGenerator::new_dedup`:
//!
//! ```rust
//! let mut cg = CardGenerator::new_dedup(card_pool, duplicate);
//! ```

use rand::rngs::ThreadRng;
use rand::Rng;

/// Basic card generator that disregards rarities. Supports limiting the number of times a card can
/// be generated.
pub struct CardGenerator {
    /// Card pool to generate cards from.
    cards: Vec<u32>,
    /// Number of times each card is left in the card pool. If `None` any card can show up an
    /// arbitrary number of times.
    card_quantities: Option<Vec<usize>>,
    /// Number of *unique* cards remaining in the card pool.
    remaining_cards: usize,
    rng: ThreadRng,
}

impl CardGenerator {
    /// Constructs a new [`CardGenerator`].
    ///
    /// # Arguments
    ///
    /// * `cards` – The card pool of the card generator.
    /// * `dups` – The number of times each card can be generated. Set to `0` for no limit.
    pub fn new(cards: Vec<u32>, dups: usize) -> CardGenerator {
        let remaining_cards = cards.len();

        let quants = if dups > 0 {
            Some(vec![dups; remaining_cards])
        } else {
            None
        };

        let card_gen = CardGenerator {
            cards: cards,
            card_quantities: quants,
            remaining_cards: remaining_cards,
            rng: rand::thread_rng(),
        };

        return card_gen;
    }

    /// Constructs a [`CardGenerator`], but de-duplicates the card pool first. Same arguments as
    /// [`new`][`CardGenerator::new`].
    pub fn new_dedup(cards: Vec<u32>, dups: usize) -> CardGenerator {
        let mut cards_mut = cards.to_vec();
        cards_mut.sort_unstable();
        cards_mut.dedup();

        return CardGenerator::new(cards_mut, dups);
    }

    /// # Arguments
    ///
    /// * `n` – Number of cards to generate.
    pub fn generate(&mut self, n: usize) -> Vec<u32> {
        let mut generated = Vec::new();

        for _ in 0..n {
            let rand_num: usize = self.rng.gen_range(0..self.remaining_cards);
            generated.push(self.cards[rand_num]);

            if self.card_quantities.is_some() {
                let new_val = self.card_quantities.as_mut().unwrap().remove(rand_num) - 1;
                self.card_quantities
                    .as_mut()
                    .unwrap()
                    .insert(rand_num, new_val);

                if new_val < 1 {
                    self.remaining_cards -= 1;
                    self.cards.remove(rand_num);
                    self.card_quantities.as_mut().unwrap().remove(rand_num);
                }
            }
        }

        return generated;
    }

    /// Generates cards, like [`generate`][`CardGenerator::generate`], but splits them into multiple
    /// parts.
    ///
    /// # Arguments
    ///
    /// * `batches` – Number of batches to generate. This is equal to `result.len()`.
    ///
    /// * `cards_per_batch` – Number of cards generated for each batch.
    pub fn batch_generate(&mut self, batches: usize, cards_per_batch: usize) -> Vec<Vec<u32>> {
        (0..batches)
            .map(|_| self.generate(cards_per_batch))
            .collect()
    }
}

#[cfg(test)]
mod tests;
