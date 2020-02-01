// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Tools and abstraction on runtime storage.

pub mod top;
pub mod top_hashed;
pub mod child;
pub mod generator;

#[deprecated(note="Use module `top` instead")]
pub use top as unhashed;
#[deprecated(note="Use module `top_hashed` instead")]
pub use top_hashed as hashed;

pub use generator::{
	StorageValue, StorageMap, StorageLinkedMap, StorageDoubleMap, StoragePrefixedMap
};
