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

/// Conversion of banlist data from [anihelp](https://ygo.anihelp.co.uk/).
pub mod banlists;
/// Conversion of card data from the [YGOPRODECK API](https://ygoprodeck.com/api-guide/).
pub mod cardinfo;
/// Conversion of set/product from the [YGOPRODECK API](https://ygoprodeck.com/api-guide/).
pub mod cardsets;
/// External image management.
pub mod image_dl;
/// [YGOPRODECK API](https://ygoprodeck.com/api-guide/) database version.
pub mod vercheck;
