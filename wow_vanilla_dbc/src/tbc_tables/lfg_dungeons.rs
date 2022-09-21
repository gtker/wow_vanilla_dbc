use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::tbc_tables::faction::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LFGDungeons {
    pub rows: Vec<LFGDungeonsRow>,
}

impl DbcTable for LFGDungeons {
    type Row = LFGDungeonsRow;

    fn filename() -> &'static str { "LFGDungeons.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LFGDungeons) int32
            let id = LFGDungeonsKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc
            let name_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // type_id: int32
            let type_id = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture_filename: string_ref
            let texture_filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // expansion_level: int32
            let expansion_level = crate::util::read_i32_le(chunk)?;


            rows.push(LFGDungeonsRow {
                id,
                name_lang,
                min_level,
                max_level,
                type_id,
                faction,
                texture_filename,
                expansion_level,
            });
        }

        Ok(LFGDungeons { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (LFGDungeons) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // type_id: int32
            b.write_all(&row.type_id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // texture_filename: string_ref
            if !row.texture_filename.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture_filename.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // expansion_level: int32
            b.write_all(&row.expansion_level.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for LFGDungeons {
    type PrimaryKey = LFGDungeonsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl LFGDungeons {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            if !row.texture_filename.is_empty() { b.write_all(row.texture_filename.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            if !row.texture_filename.is_empty() { sum += row.texture_filename.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct LFGDungeonsKey {
    pub id: i32
}

impl LFGDungeonsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name_lang: LocalizedString,
    pub min_level: i32,
    pub max_level: i32,
    pub type_id: i32,
    pub faction: FactionKey,
    pub texture_filename: String,
    pub expansion_level: i32,
}
