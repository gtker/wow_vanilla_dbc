use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::spell_item_enchantment::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GemProperties {
    pub rows: Vec<GemPropertiesRow>,
}

impl DbcTable for GemProperties {
    type Row = GemPropertiesRow;

    fn filename() -> &'static str { "GemProperties.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (GemProperties) int32
            let id = GemPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // enchant_id: foreign_key (SpellItemEnchantment) int32
            let enchant_id = SpellItemEnchantmentKey::new(crate::util::read_i32_le(chunk)?.into());

            // maxcount_inv: int32
            let maxcount_inv = crate::util::read_i32_le(chunk)?;

            // maxcount_item: int32
            let maxcount_item = crate::util::read_i32_le(chunk)?;

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;


            rows.push(GemPropertiesRow {
                id,
                enchant_id,
                maxcount_inv,
                maxcount_item,
                ty,
            });
        }

        Ok(GemProperties { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (GemProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // enchant_id: foreign_key (SpellItemEnchantment) int32
            b.write_all(&(row.enchant_id.id as i32).to_le_bytes())?;

            // maxcount_inv: int32
            b.write_all(&row.maxcount_inv.to_le_bytes())?;

            // maxcount_item: int32
            b.write_all(&row.maxcount_item.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GemProperties {
    type PrimaryKey = GemPropertiesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GemPropertiesKey {
    pub id: i32
}

impl GemPropertiesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct GemPropertiesRow {
    pub id: GemPropertiesKey,
    pub enchant_id: SpellItemEnchantmentKey,
    pub maxcount_inv: i32,
    pub maxcount_item: i32,
    pub ty: i32,
}
