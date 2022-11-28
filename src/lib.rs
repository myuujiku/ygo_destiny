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

//! # YGO Destiny
//!
//! This library crate contains all modules used by the ygo_destiny binary.

/// Back end module.
pub mod logic;
/// Front end module.
pub mod ui;

/// Reverse DNS style application identifier: `com.myujiku.ygo_destiny`.
pub const APP_ID: &str = "com.myujiku.ygo_destiny";
