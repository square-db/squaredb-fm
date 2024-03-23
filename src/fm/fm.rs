use std:: {
  fs:: {
    self,
    File,
    OpenOptions
  },
  io:: {
    BufReader,
    BufWriter,
    Read,
    Write
  },
  path:: {
    Path
  }
};
use crate::err::err:: {
  FmError
};

pub struct FsApi;

pub trait FsApiTrait {
  fn ddel(path: &Path, force: bool) -> Result<(),
  FmError>;
  fn exist(path: &Path) -> Result<(),
  FmError>;
  fn read(path: &Path) -> Result<String,
  FmError>;
  fn write(path: &Path, content: &str) -> Result<(),
  FmError>;
  fn rename(path: &Path, new_path: &Path) -> Result<(),
  FmError>;
  fn create_dir(path: &Path) -> Result<(),
  FmError>;
  fn read_dir(path: &Path) -> Result<Vec<String>,
  FmError>;
}

impl FsApiTrait for FsApi {
  fn read(path: &Path) -> Result<String,
  FmError> {
    let mut buffer = Vec::new();

    if let Ok(file) = File::open(path) {
      if let Ok(_) = BufReader::new(file).read_to_end(&mut buffer) {
        if let Ok(contents) = String::from_utf8(buffer) {
          return Ok(contents);
        } else {
          return Err(FmError::Utf8Error);
        }
      } else {
        return Err(FmError::OsError);
      }
    } else {
      return Err(FmError::OsError);
    }
  }

  fn write(path: &Path, content: &str) -> Result<(),
  FmError> {
    let parent_dir = path.parent().ok_or(FmError::OsError)?;
    fs::create_dir_all(parent_dir).map_err(|_| FmError::OsError)?;
    
    let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .map_err(|_| FmError::OsError)?;
    let mut writer = BufWriter::new(file);
    writer
    .write_all(content.as_bytes())
    .map_err(|_| FmError::IoError)?;
    writer.flush().map_err(|_| FmError::IoError)?;
    Ok(())
  }

  fn ddel(path: &Path, force: bool) -> Result<(),
  FmError> {
    let remove_result = if force {
      fs::remove_dir_all(path)
    } else {
      fs::remove_dir(path)
    };
    remove_result.map_err(|_| FmError::OsError)
  }

  fn exist(path: &Path) -> Result<(),
  FmError> {
    fs::metadata(path).map(|_| ()).map_err(|_| FmError::EofError)
  }

  fn create_dir(path: &Path) -> Result<(),
  FmError> {
    fs::create_dir_all(path).map_err(|_| FmError::OsError)
  }

  fn read_dir(path: &Path) -> Result<Vec<String>,
  FmError> {
    fs::read_dir(path)
    .map_err(|_| FmError::OsError)
    .map(|entries| {
      entries
      .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
      .collect()
    })
  }

  fn rename(path: &Path, new_path: &Path) -> Result<(),
  FmError> {
    fs::rename(path, new_path).map_err(|_| FmError::OsError)
  }
}