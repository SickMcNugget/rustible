pub mod add_host;
pub mod apt;
pub mod archive;
pub mod git;
pub mod read;

pub type Result<T> = std::result::Result<T, ModuleError>;

#[derive(Debug)]
pub enum ModuleError {
    Other,
    IOError(std::io::Error),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error in module")
    }
}

impl std::error::Error for ModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Other => None,
            Self::IOError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        ModuleError::IOError(value)
    }
}
