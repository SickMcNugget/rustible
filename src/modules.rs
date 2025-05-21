pub mod add_host;
pub mod apt;
pub mod archive;
pub mod git;

pub type Result<T> = std::result::Result<T, ModuleError>;

#[derive(Debug)]
pub enum ModuleError {
    PlainMessage(String),
    IOError(std::io::Error),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlainMessage(msg) => write!(f, "{msg}"),
            Self::IOError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::PlainMessage(_) => None,
            Self::IOError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        ModuleError::IOError(value)
    }
}
