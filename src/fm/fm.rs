/* This defines all possible low level operstions to a file
*/

use std::fs:: {
  self,
  File
};
use std::io:: {
  BufReader,
  BufWriter,
  Read,
  Write
};
use std::path::Path;
pub struct FsApi;

pub trait FsApiTrait {
  fn ddel(path: &str, force: bool) -> Result<(),
  String>;
  fn fdel(path: &str) -> Result<(),
  String>;
  fn exsit(path: &str) -> Result<(),
  String>;
  fn read(path: &str) -> Result<String,
  String>;
  fn write(path: &str, content: &[u8]) -> Result<(),
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
    let file = File::open(path).map_err(|err| format!("OS error: {}", err))?;
    let reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.bytes().for_each(|byte| buffer.push(byte.unwrap()));

    Ok(String::from_utf8_lossy(&buffer).to_string())
  }

  fn write(path: &str, content: &[u8]) -> Result<(),
  String> {
    let parent_dir = std::path::Path::new(path).parent().ok_or_else( || "OS error: Failure when attempting to write!".to_string())?;

    if !parent_dir.exists() {
      fs::create_dir_all(parent_dir).map_err(|err| format!("OS error: {}", err))?;
    }

    let file = File::create(path).map_err(|err| format!("OS error: {}", err))?;
    let mut writer = BufWriter::new(file);

    writer.write_all(content).map_err(|err| format!("OS error: {}", err))
  }

  fn fdel(path: &str) -> Result<(),
  String> {
    fs::remove_file(path).map_err(|err| format!("OS error: {}", err))
  }

  fn ddel(path: &str, force: bool) -> Result<(),
  String> {
    if !force {
      fs::remove_dir(path).map_err(|err| format!("OS error: {}", err))
    } else {
      fs::remove_dir_all(path).map_err(|err| format!("OS error: {}", err))
    }
  }

  fn exsit(path: &str) -> Result<(),
  String> {
    fs::metadata(path).map_err(|_| "Eof Error: Cannot find it on OS level!".to_string()).map(|_| ())
  }

  fn create_dir(path: &str) -> Result<(),
  String> {

    fs::create_dir_all(path).map_err(|err| format!("OS error: {}", err))?;

    Ok(())
  }

  fn read_dir(path: &str) -> Result<Vec<String>,
  String> {
    fs::read_dir(path).map_err(|_| "OS Error: Cannot read sub dirs!".to_string())
    .map(|entries| entries.filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok())).collect())
  }

  fn rename(path: &str, new_path: &str) -> Result<(),
  String> {
    let source = Path::new(path);
    let destination = Path::new(new_path);

    if source.exists() && source.is_dir() {
      // Check if the destination directory already exists
      if destination.exists() {
        return Err("Destination directory already exists.".to_string());
      }

      // Rename the directory
      fs::rename(source, destination).map_err(|err| format!("OS error: {}", err))?;
      Ok(())
    } else {
      Err("Source directory does not exist.".to_string())
    }
  }
}