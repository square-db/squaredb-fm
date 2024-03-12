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
  path::Path
};

pub struct FsApi;

pub trait FsApiTrait {
  fn ddel(path: &str, force: bool) -> Result<(),
  String>;
  fn exist(path: &str) -> Result<(),
  String>;
  fn read(path: &str) -> Result<String,
  String>;
  fn write(path: &str, content: &str) -> Result<(),
  String>;
  fn rename(path: &str, new_path: &str) -> Result<(),
  String>;
  fn create_dir(path: &str) -> Result<(),
  String>;
  fn read_dir(path: &str) -> Result<Vec<String>,
  String>;
}

impl FsApiTrait for FsApi {

  fn read(path: &str) -> Result<String,
  String> {
    let mut buffer = Vec::new();
    BufReader::new(File::open(path).map_err(|err| "OS error: ".to_owned() + &err.to_string())?)
    .read_to_end(&mut buffer)
    .map(|_| String::from_utf8_lossy(&buffer).to_string())
    .map_err(|_| "OS Error: Cannot read file!".to_string())
  }

  fn write(path: &str, content: &str) -> Result<(),
  String> {
    let parent_dir = Path::new(path).parent().ok_or_else( || "OS error: Failure when attempting to write!".to_string())?;
    fs::create_dir_all(parent_dir).map_err(|err| "OS error: ".to_owned() + &err.to_string())?;

    OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .map_err(|err| "OS error: ".to_owned() + &err.to_string())
    .and_then(|file| BufWriter::new(file).write_all(content.as_bytes()).map_err(|err| "OS error: ".to_owned() + &err.to_string()))
  }

  fn ddel(path: &str, force: bool) -> Result<(),
  String> {
    let remove_result = if force {
      fs::remove_dir_all(path)
    } else {
      fs::remove_dir(path)
    };
    remove_result.map_err(|err| "OS error: ".to_owned() + &err.to_string())
  }

  fn exist(path: &str) -> Result<(),
  String> {
    fs::metadata(path).map(|_| ()).map_err(|_| "Eof Error: Cannot find it on OS level!".to_string())
  }

  fn create_dir(path: &str) -> Result<(),
  String> {
    fs::create_dir_all(path).map_err(|err| "OS error: ".to_owned() + &err.to_string()).map(|_| ())
  }

  fn read_dir(path: &str) -> Result<Vec<String>,
  String> {
    fs::read_dir(path)
    .map_err(|_| "OS Error: Cannot read sub dirs!".to_string())
    .map(|entries| entries.filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok())).collect())
  }

  fn rename(path: &str, new_path: &str) -> Result<(),
  String> {
    if let Err(err) = fs::rename(path, new_path) {
      Err("OS error: ".to_owned() + &err.to_string())
    } else {
      Ok(())
    }
  }
}