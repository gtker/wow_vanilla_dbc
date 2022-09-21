use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq)]
pub struct PetPersonality {
    pub rows: Vec<PetPersonalityRow>,
}

impl DbcTable for PetPersonality {
    type Row = PetPersonalityRow;

    fn filename() -> &'static str { "PetPersonality.dbc" }

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

            // id: primary_key (PetPersonality) int32
            let id = PetPersonalityKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc
            let name_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // happiness_threshold: int32[3]
            let happiness_threshold = crate::util::read_array_i32::<3>(chunk)?;

            // happiness_damage: float[3]
            let happiness_damage = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name_lang,
                happiness_threshold,
                happiness_damage,
            });
        }

        Ok(PetPersonality { rows, })
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
            // id: primary_key (PetPersonality) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // happiness_threshold: int32[3]
            for i in row.happiness_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // happiness_damage: float[3]
            for i in row.happiness_damage {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for PetPersonality {
    type PrimaryKey = PetPersonalityKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl PetPersonality {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct PetPersonalityKey {
    pub id: i32
}

impl PetPersonalityKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name_lang: LocalizedString,
    pub happiness_threshold: [i32; 3],
    pub happiness_damage: [f32; 3],
}

