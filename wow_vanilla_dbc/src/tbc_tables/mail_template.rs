use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq)]
pub struct MailTemplate {
    pub rows: Vec<MailTemplateRow>,
}

impl DbcTable for MailTemplate {
    type Row = MailTemplateRow;

    fn filename() -> &'static str { "MailTemplate.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 76 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 76,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 19 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 19,
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

            // id: primary_key (MailTemplate) int32
            let id = MailTemplateKey::new(crate::util::read_i32_le(chunk)?);

            // subject_lang: string_ref_loc
            let subject_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // body_lang: string_ref_loc
            let body_lang = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(MailTemplateRow {
                id,
                subject_lang,
                body_lang,
            });
        }

        Ok(MailTemplate { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 19,
            record_size: 76,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (MailTemplate) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // subject_lang: string_ref_loc
            b.write_all(&row.subject_lang.string_indices_as_array(&mut string_index))?;

            // body_lang: string_ref_loc
            b.write_all(&row.body_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for MailTemplate {
    type PrimaryKey = MailTemplateKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl MailTemplate {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.subject_lang.string_block_as_array(b)?;
            row.body_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.subject_lang.string_block_size();
            sum += row.body_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct MailTemplateKey {
    pub id: i32
}

impl MailTemplateKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct MailTemplateRow {
    pub id: MailTemplateKey,
    pub subject_lang: LocalizedString,
    pub body_lang: LocalizedString,
}
