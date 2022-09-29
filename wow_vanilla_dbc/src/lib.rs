//! DEPRECATED. USE [wow_dbc](https://github.com/gtker/wow_vanilla_dbc) INSTEAD.

#![forbid(unsafe_code)]
// This requires fields knowing about the sizes of enums
#![allow(clippy::useless_conversion)]
#![warn(
    clippy::perf,
    clippy::correctness,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::unseparated_literal_suffix,
    missing_docs
)]

use std::io::{Read, Write};

pub(crate) mod error;
pub(crate) mod header;
#[allow(missing_docs)]
pub mod tables;
mod util;

pub use error::*;

/// DBCs from the English version of the game will only have English version strings, while other localizations will have other languages.
///
/// You are most likely interested in, [`LocalizedString::en_gb`], the English version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedString {
    /// English, Great Britain
    pub en_gb: String,
    /// Korean, Korea
    pub ko_kr: String,
    /// French, France
    pub fr_fr: String,
    /// German, Germany
    pub de_de: String,
    /// English, China
    pub en_cn: String,
    /// English, Taiwan
    pub en_tw: String,
    /// Spanish, Spain
    pub es_es: String,
    /// Spanish, Mexico
    pub es_mx: String,
    /// Unknown flags.
    pub flags: u32,
}

impl LocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        en_gb: String,
        ko_kr: String,
        fr_fr: String,
        de_de: String,
        en_cn: String,
        en_tw: String,
        es_es: String,
        es_mx: String,
        flags: u32,
    ) -> Self {
        Self {
            en_gb,
            ko_kr,
            fr_fr,
            de_de,
            en_cn,
            en_tw,
            es_es,
            es_mx,
            flags,
        }
    }

    pub(crate) fn string_indices_as_array(&self, string_index: &mut usize) -> [u8; 36] {
        let mut arr = [0_u8; 36];
        let mut index = 0;

        for s in self.strings() {
            let value = (if !s.is_empty() {
                let v = *string_index;
                *string_index += s.len() + 1;
                v
            } else {
                0
            } as u32)
                .to_le_bytes();

            arr[index] = value[0];
            arr[index + 1] = value[1];
            arr[index + 2] = value[2];
            arr[index + 3] = value[3];
            index += 4;
        }

        let value = &self.flags.to_le_bytes();
        arr[index] = value[0];
        arr[index + 1] = value[1];
        arr[index + 2] = value[2];
        arr[index + 3] = value[3];

        arr
    }

    pub(crate) fn string_block_as_array(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        for s in self.strings() {
            if !s.is_empty() {
                b.write_all(s.as_bytes())?;
                b.write_all(&[0])?;
            };
        }

        Ok(())
    }

    pub(crate) fn string_block_size(&self) -> usize {
        let mut sum = 0;

        for s in self.strings() {
            if !s.is_empty() {
                sum += s.len() + 1;
            }
        }

        sum
    }

    pub(crate) const fn strings(&self) -> [&String; 8] {
        [
            &self.en_gb,
            &self.ko_kr,
            &self.fr_fr,
            &self.de_de,
            &self.en_cn,
            &self.en_tw,
            &self.es_es,
            &self.es_mx,
        ]
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl TryFrom<i32> for Gender {
    type Error = InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryFrom::try_from(value as i8)
    }
}

impl TryFrom<i8> for Gender {
    type Error = InvalidEnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Male,
            1 => Self::Female,
            val => return Err(InvalidEnumError::new("Gender", val as i64)),
        })
    }
}

impl Gender {
    const fn as_int(&self) -> i32 {
        match self {
            Self::Male => 0,
            Self::Female => 1,
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SizeClass {
    None,
    Small,
    Medium,
    Large,
    Giant,
    Colossal,
}

impl TryFrom<i32> for SizeClass {
    type Error = InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            -1 => Self::None,
            0 => Self::Small,
            1 => Self::Medium,
            2 => Self::Large,
            3 => Self::Giant,
            4 => Self::Colossal,
            val => return Err(InvalidEnumError::new("SizeClass", val as i64)),
        })
    }
}

impl SizeClass {
    const fn as_int(&self) -> i32 {
        match self {
            Self::None => -1,
            Self::Small => 0,
            Self::Medium => 1,
            Self::Large => 2,
            Self::Giant => 3,
            Self::Colossal => 4,
        }
    }
}

impl Default for SizeClass {
    fn default() -> Self {
        Self::None
    }
}

/// Main trait for the crate. Implemented by all tables in [`tables`].
pub trait DbcTable: Sized {
    /// Will be the name of the implementing type suffixed with `Row`.
    type Row;

    /// The name of the DBC file _with_ `.dbc` at the end.
    fn filename() -> &'static str;

    /// Array of all rows. Are not guaranteed to be in any order.
    fn rows(&self) -> &[Self::Row];
    /// Mutable array of all rows. Are not guaranteed to be in any order.
    fn rows_mut(&mut self) -> &mut [Self::Row];

    /// Read table from bytes.
    ///
    /// Will error with [`InvalidHeaderError`] if the magic numbers (`0x43424457`) at the start of the file do not match.
    fn read(b: &mut impl Read) -> Result<Self, DbcError>;
    /// Write to bytes.
    ///
    /// The string block will always start with a zero byte so that a string index of 0 is always an empty string.
    ///
    /// This is not guaranteed to create the exact same binary as is shipped with the game, but it will be semantically the same.
    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error>;
}

/// Implemented by tables that have a primary key.
///
/// This is a separate trait instead of just implementing [`std::ops::Index`] and [`std::ops::IndexMut`] since
/// those traits do not return [`Option`]s and only have the possibility of panicking on invalid keys.
///
/// The original DBCs do not really respect primary/foreign keys, so this just seemed like it would make everything more annoying.
pub trait Indexable: DbcTable {
    /// Key used to index into the table. Same name as the table suffixed with `Key`.
    type PrimaryKey;

    /// Gets the primary key, if present. Internally this is just [`std::iter::Iterator::find`] since the
    /// items are not guaranteed to be ordered nor even be present.
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row>;

    /// Gets the primary key, if present. Internally this is just [`std::iter::Iterator::find`] since the
    /// items are not guaranteed to be ordered nor even be present.
    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row>;
}
