use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct AreaGroup {
    pub rows: Vec<AreaGroupRow>,
}

impl DbcTable for AreaGroup {
    type Row = AreaGroupRow;

    fn filename() -> &'static str { "AreaGroup.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (AreaGroup) int32
            let id = AreaGroupKey::new(crate::util::read_i32_le(chunk)?);

            // area_id: int32[6]
            let area_id = crate::util::read_array_i32::<6>(chunk)?;

            // next_area_id: foreign_key (AreaGroup) int32
            let next_area_id = AreaGroupKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(AreaGroupRow {
                id,
                area_id,
                next_area_id,
            });
        }

        Ok(AreaGroup { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (AreaGroup) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // area_id: int32[6]
            for i in row.area_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // next_area_id: foreign_key (AreaGroup) int32
            b.write_all(&(row.next_area_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for AreaGroup {
    type PrimaryKey = AreaGroupKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct AreaGroupKey {
    pub id: i32
}

impl AreaGroupKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaGroupRow {
    pub id: AreaGroupKey,
    pub area_id: [i32; 6],
    pub next_area_id: AreaGroupKey,
}

