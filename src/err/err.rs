use thiserror::Error;

#[derive(Error, Debug)]
pub enum FmError {
    #[error("Disk Error: Cannot read data from Memory!")]
    MemoryReadError,
    #[error("Disk Info: Cannot not find data on OS level!")]
    NotFound,
    #[error("Disk Info: Cannot not find Table on OS level!")]
    TableNotFound,
    #[error("Disk Error: Cannot not delete Table on OS level!")]
    TableDeletionError,
    #[error("Disk Error: Cannot not create Table on OS level!")]
    TableCreationError,
    #[error("Disk Error: Cannot not read Table on OS level!")]
    TableReadError,
    #[error("Disk Error: Cannot not deserialize Table on OS level!")]
    TableDeserializationError,
    #[error("Disk Error: Cannot not create Database on OS level!")]
    DatabaseCreationError,
    #[error("Disk Error: Cannot not read Database on OS level!")]
    DatabaseReadError,
    #[error("Disk Error: Cannot not delete Database on OS level!")]
    DatabaseDeletionError,
    #[error("Disk Error: Cannot not rename Database on OS level!")]
    DatabaseRenameError,
    #[error("Disk Error: Cannot not find Database on OS level!")]
    DatabaseNotFound,
    #[error("Decryption Error: Cannot decrypt data!")]
    DecryptionError,
    #[error("Encryption Error: Cannot encrypt data!")]
    EncryptionError,
    #[error("OS Error: Opening File on OS level Failed!")]
    OsError,
    #[error("Conversion Error: Converting data to UTF-8 Failed!")]
    Utf8Error,
    #[error("IO Error: Reading/Writing data Failed!")]
    IoError,
    #[error("Eof Error: Couldnot open file Failed!")]
    EofError,
}