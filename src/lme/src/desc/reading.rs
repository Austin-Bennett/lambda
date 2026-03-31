use std::fs;
use std::ops::Deref;
use std::path::Path;
use crate::desc::{LMEFunctionLocation, LME};
use crate::util::bytereader::LEByteReader;

impl LME {

    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file_data = fs::read(path)?;

        Self::from_memory(file_data)
    }

    pub fn from_memory(file_data: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let mut reader = LEByteReader::new(file_data.as_ref());
        //ensure the file starts with LME (0x4c, 0x4d, 0x45)
        if let Some( 0x4c ) = reader.next_byte() &&
           let Some( 0x4d ) = reader.next_byte() &&
            let Some( 0x45 ) = reader.next_byte() {

        } else {
            return Err(anyhow::Error::msg("Invalid file data, expected prefix LME ( 0x4c4d45 )"));
        }

        let major = reader.next_dword().ok_or(anyhow::Error::msg("Expected major revision number after LME prefix"))?;
        let minor = reader.next_dword().ok_or(anyhow::Error::msg("Expected minor revision number after major revision"))?;
        let hotfix = reader.next_dword().ok_or(anyhow::Error::msg("Expected hotfix revision number after minor revision"))?;

        //we do not need the lookup section right now, as we just have the metadata and code sections right after, so we will skip it
        let num_lookups = reader.next_qword().ok_or(anyhow::Error::msg("Expected lookup section length after file length"))?;

        for i in 0..num_lookups {
            //skip the id and position

            reader.next_dword().ok_or(anyhow::Error::msg(format!("Didnt expect EOF, expected ({})th section id", i)))?;
            reader.next_qword().ok_or(anyhow::Error::msg(format!("Didnt expect EOF, expected ({})th section location", i)))?;
        }

        //the part we care the most about besides code: the metadata, this is needed to run the actual program and for debugging
        let entry = reader.next_qword().ok_or(anyhow::Error::msg("Expected program entry location after section lookup"))?;
        let num_funcs = reader.next_qword().ok_or(anyhow::Error::msg("Expected function count after entry location"))?;

        let mut funcs = Vec::new();
        for i in 0..num_funcs {
            let name_len = reader.next_qword().ok_or(anyhow::Error::msg("Expected function name"))?;
            let name = reader.next_str(name_len as usize).ok_or(anyhow::Error::msg("Expected function name"))?.to_string();
            let loc = reader.next_qword().ok_or(anyhow::Error::msg("Expected function label location"))?;

            funcs.push(
                LMEFunctionLocation{
                    name,
                    loc
                }
            )
        }


        //the rest is bytecode data
        let bc = Vec::from(reader.deref());


        Ok(LME{
            code_entry: entry,
            functions: funcs,
            code: bc,
        })
    }
}