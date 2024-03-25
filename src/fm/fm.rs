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
  fn open_dst(path: &Path) -> Result<fs::File,
  FmError>;
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
  fn open_dst(path: &Path) -> Result<fs::File,
  FmError> {
    let file_result = OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open(path);

    match file_result {
      Ok(file) => Ok(file),
      Err(err) => Err(FmError::OsError(err.to_string())),
    }
  }

  fn read(path: &Path) -> Result<String,
  FmError> {
    let mut buffer = Vec::new();
    match File::open(path) {
      Ok(file) => {
        match BufReader::new(file).read_to_end(&mut buffer) {
          Ok(_) => {
            if let Ok(contents) = String::from_utf8(buffer) {
              return Ok(contents);
            } else {
              return Err(FmError::Utf8Error);
            }
          },
          Err(err) => Err(FmError::OsError(err.to_string()))
        }
      },
      Err(err) => Err(FmError::OsError(err.to_string()))
    }
  }

  fn write(path: &Path, content: &str) -> Result<(),
  FmError> {
    let parent_dir = match path.parent() {
      Some(res) => res,
      None => todo!()
    };
    
    if let Err(err) = fs::create_dir_all(parent_dir) {
      return Err(FmError::OsError(err.to_string()));
    };
    let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .map_err(|err| FmError::OsError(err.to_string()))?;
    let mut writer = BufWriter::new(file);
    writer
    .write_all(content.as_bytes())
    .map_err(|err| FmError::IoError(err.to_string()))?;
    writer.flush().map_err(|err| FmError::IoError(err.to_string()))?;
    Ok(())
  }

  fn ddel(path: &Path, force: bool) -> Result<(),
  FmError> {
    let remove_result = if force {
      fs::remove_dir_all(path)
    } else {
      fs::remove_dir(path)
    };
    remove_result.map_err(|err| FmError::OsError(err.to_string()))
  }

  fn exist(path: &Path) -> Result<(),
  FmError> {
    fs::metadata(path).map(|_| ()).map_err(|_| FmError::EofError)
  }

  fn create_dir(path: &Path) -> Result<(),
  FmError> {
    fs::create_dir_all(path).map_err(|err| FmError::OsError(err.to_string()))
  }

  fn read_dir(path: &Path) -> Result<Vec<String>,
  FmError> {
    fs::read_dir(path)
    .map_err(|err| FmError::OsError(err.to_string()))
    .map(|entries| {
      entries
      .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
      .collect()
    })
  }

  fn rename(path: &Path, new_path: &Path) -> Result<(),
  FmError> {
    fs::rename(path, new_path).map_err(|err| FmError::OsError(err.to_string()))
  }
}