//! 한 번에 한 단계씩 파일을 모사한다.

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// 아마도 파일 시스템에 있을 '파일'을 나타낸다.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

pub trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err("File must be open for reading".into());
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// 빈 `File`을 새로 만든다.
    ///
    /// # 예제
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &[u8]) -> File {
        let mut f = File::new(name);
        f.data = data.to_owned();

        f
    }

    /// 파일 길이를 바이트로 반환한다.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 파일 이름을 반환한다.
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;

    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;

    Ok(f)
}
