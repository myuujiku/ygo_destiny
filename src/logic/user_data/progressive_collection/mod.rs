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

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

/// Action executed by a [`Change`] in a [`ProgressiveCollection`].
#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq)]
pub enum Action {
    #[default]
    None,
    Add,
    Remove,
}

/// Will contain more variants in the future.
#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq)]
pub enum DraftType {
    #[default]
    ChoiceDraft,
}

/// Wrapper for a card in a collection. Will contain more fields, like rarity, in the future.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub id: u32,
}

/// A modification that is applied to a [`ProgressiveCollection`].
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Change {
    pub action: Action,
    pub cards: Vec<Card>,
    pub date: String,
    pub round: Option<u16>,
}

/// Container for settings related to drafting.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DraftSettings {
    pub draft_type: DraftType,
    pub rounds_num: usize,
    pub cards_num: usize,
    pub follow_sets: bool,
    pub sets: Option<Vec<String>>,
    pub rotate_sets: usize,
    pub allow_undo: bool,
    // ChoiceDraft
    pub choices_num: usize,
    pub selections_num: usize,
}

/// Container for additional information about the draft.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DraftStatus {
    pub round: u16,
    pub wins: u16,
    pub losses: u16,
}

/// Data that should be accessible without having to read the whole file.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MetaData {
    pub name: String,
    pub description: String,
    pub pinned: bool,
}

/// Card collection data type holding draft settings and more.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProgressiveCollection {
    pub meta_data: MetaData,
    pub cards: HashMap<Card, u8>,
    pub changes: VecDeque<Change>,
    pub draft_settings: DraftSettings,
    pub tags: HashMap<String, Vec<Card>>,
}

impl Change {
    /// Creates a new change.
    ///
    /// # Arguments
    ///
    /// * `action` – Action to execute. If set to [`Action::None`] nothing will happen.
    /// * `cards` – Cards that are to be changed.
    /// * `date` – Date when the change was executed.
    /// * `round` – Optional. The draft round the change was executed in.
    pub fn new(
        action: Action,
        cards: Vec<Card>,
        date: String,
        round: Option<u16>,
    ) -> Self {
        Self {
            action: action,
            cards: cards,
            date: date,
            round: round,
        }
    }

    /// Returns `false` if `self.action` is [`Action::None`] and `true` otherwise.
    pub fn has_action(&self) -> bool {
        match self.action {
            Action::None => false,
            _ => true,
        }
    }

    /// Returns `true` if `self.action` is [`Action::Add`] and `false` otherwise.
    pub fn is_action_add(&self) -> bool {
        match self.action {
            Action::Add => true,
            _ => false,
        }
    }

    /// Returns `true` if `self.action` is [`Action::Remove`] and `false` otherwise.
    pub fn is_action_remove(&self) -> bool {
        match self.action {
            Action::Remove => true,
            _ => false,
        }
    }
}

impl Clone for Change {
    fn clone(&self) -> Self {
        Self::new(
            self.action,
            self.cards.to_vec(),
            self.date.to_string(),
            self.round,
        )
    }
}

impl ProgressiveCollection {
    /// Adds a new `Change` and applies it to [`cards`][`ProgressiveCollection::cards`].
    pub fn add_change(&mut self, change: Change) {
        if change.has_action() {
            if change.is_action_add() {
                self.add_cards(&change.cards);
            } else {
                self.remove_cards(&change.cards);
            }

            self.changes.push_front(change);
        }
    }

    /// Removes the most recent `Change` applies the inverse action to
    /// [`cards`][`ProgressiveCollection::cards`].
    pub fn undo_change(&mut self) {
        let change = self.changes.pop_front().unwrap();

        if change.is_action_remove() {
            self.add_cards(&change.cards);
        } else {
            self.remove_cards(&change.cards)
        }
    }

    /// # Arguments
    ///
    /// * `cards` – Reference to the cards to add.
    fn add_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                self.cards.insert(card.clone(), quantity + 1);
            } else {
                self.cards.insert(card.clone(), 1);
            }
        }
    }

    /// # Arguments
    ///
    /// * `cards` – Reference to the cards to remove.
    fn remove_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                if quantity < &1 {
                    self.cards.remove(card);
                } else {
                    self.cards.insert(card.clone(), quantity - 1);
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests;
