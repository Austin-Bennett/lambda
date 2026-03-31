use std::io::{BufWriter, Write};
use crate::desc::LME;

impl LME {
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        //write the header


        let mut header = BufWriter::new(Vec::new());
        header.write(&[0x4c, 0x4d, 0x45])?;
        header.write(&1u32.to_le_bytes())?;
        header.write(&0u32.to_le_bytes())?;
        header.write(&0u32.to_le_bytes())?;


        let mut lookup = BufWriter::new(Vec::new());

        let mut code_meta = BufWriter::new(Vec::new());

        code_meta.write(&self.code_entry.to_le_bytes())?;
        code_meta.write(&(self.functions.len() as u64).to_le_bytes())?;

        for f in &self.functions {
            code_meta.write(&(f.name.len() as u64).to_le_bytes())?;
            code_meta.write(f.name.as_bytes())?;
            code_meta.write(&f.loc.to_le_bytes())?;
        }

        let code_meta = code_meta.into_inner()?;

        /*
        4 sections
        header: offset: 0, size: 15
        lookup: offset: 15, size: 8 + 12 * length
        code_meta: offset: 15 + 8 + 12 * length, size: code_meta.len()
        code: 15 + 8 + 12 * length + code_meta.len()
        */

        let lookup_len = 4u64;

        lookup.write(&lookup_len.to_le_bytes())?;

        lookup.write(&0u32.to_le_bytes())?;
        lookup.write(&0u64.to_le_bytes())?;

        lookup.write(&1u32.to_le_bytes())?;
        lookup.write(&15u64.to_le_bytes())?;

        lookup.write(&2u32.to_le_bytes())?;
        lookup.write(&(15u64 + 8u64 + (12u64 * lookup_len)).to_le_bytes())?;

        lookup.write(&3u32.to_le_bytes())?;
        lookup.write(&(15u64 + 8u64 + (12u64 * lookup_len) + code_meta.len() as u64).to_le_bytes())?;

        //combine
        let mut res = header.into_inner()?;
        res.extend(lookup.into_inner()?);
        res.extend(code_meta);
        res.extend(&self.code);

        Ok(res)
    }
}