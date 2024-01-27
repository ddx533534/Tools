use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum FileState {
    OPEN,
    CLOSED,
    UNKNOWN,
}

#[derive(Debug)]
pub struct File<T> {
    name: String,
    data: Vec<T>,
    file_state: FileState,
}
impl<T> File<T> {
    pub fn new(name: String, data: Vec<T>, file_state: FileState) -> File<T> {
        File {
            name: name,
            data: data,
            file_state: file_state,
        }
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::CLOSED => write!(f, "OPEN"),
            FileState::OPEN => write!(f, "CLOSED"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl<T: Debug> Display for File<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{:?},{}>", self.name, self.data, self.file_state)
    }
}
