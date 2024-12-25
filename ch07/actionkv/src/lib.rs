use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::Crc;
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> Result<ActionKV, io::Error> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;

        let index = HashMap::new();

        Ok(ActionKV { f, index })
    }

    fn process_record<R: Read>(f: &mut R) -> Result<KeyValuePair, io::Error> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;

        let mut data = ByteString::with_capacity(data_len as usize);
        f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&data);
        if checksum != saved_checksum {
            panic!("data corruption encountered ({checksum:08x} != {saved_checksum:08x})");
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn seek_to_end(&mut self) -> Result<u64, io::Error> {
        self.f.seek(io::SeekFrom::End(0))
    }

    pub fn load(&mut self) -> Result<(), io::Error> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let current_pos = f.stream_position()?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => break,
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, current_pos);
        }

        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> Result<Option<ByteString>, io::Error> {
        let position = match self.index.get(key) {
            Some(position) => *position,
            None => return Ok(None),
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> Result<KeyValuePair, io::Error> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(io::SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;

        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> Result<Option<(u64, ByteString)>, io::Error> {
        let mut f = BufReader::new(&mut self.f);
        let mut found = None;

        loop {
            let position = f.stream_position()?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => break,
                    _ => return Err(err),
                },
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }

            // 키를 덮어쓸 경우를 대비해 파일의 끝까지 반복하는 것이 중요하다.
        }

        Ok(found)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> Result<(), io::Error> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);

        Ok(())
    }

    pub fn insert_but_ignore_index(
        &mut self,
        key: &ByteStr,
        value: &ByteStr,
    ) -> Result<u64, io::Error> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        tmp.extend(key);
        tmp.extend(value);
        let checksum = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.stream_position()?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> Result<(), io::Error> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> Result<(), io::Error> {
        self.insert(key, b"")
    }
}
