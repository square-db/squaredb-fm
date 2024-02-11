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
  fn get_f_size(path: &str) -> Result<u64,
  String>;
  fn append_file(path: &str, content: &str) -> Result<(),
  String>;
  fn update_file(path: &str, index: usize, new_content: Vec<u8>) -> Result<(),
  String>;
  fn delete_content_in_file(path: &str, index: usize) -> Result<(),
  String>;
  fn ddel(path: &str, force: bool) -> Result<(),
  String>;
  fn fdel(path: &str) -> Result<(),
  String>;
  fn exist(path: &str) -> Result<(),
  String>;
  fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(),
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
  fn append_file(path: &str, content: &str) -> Result<(),
  String> {
    OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(path)
    .map_err(|_| "OS Error: Cannot open or append file!".to_string())
    .and_then(|file| BufWriter::new(file).write_all(content.as_bytes()).map_err(|_| "OS Error: Cannot write to file!".to_string()))
  }

  fn get_f_size(path: &str) -> Result<u64,
  String> {
    fs::metadata(path).map(|metadata| metadata.len()).map_err(|_| "OS Error: Cannot get file size!".to_string())
  }

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

  fn fdel(path: &str) -> Result<(),
  String> {
    fs::remove_file(path).map_err(|err| "OS error: ".to_owned() + &err.to_string())
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

  fn update_file(path: &str, index: usize, new_content: Vec<u8>) -> Result<(),
  String> {
    match FsApi::read(path) {
      Ok(data) => {
        let bytes: Vec<u8> = data.as_bytes().to_vec();
        let mut records: Vec<Vec<u8>> = bytes.split(|&byte| byte == b'\n')
        .map(|slice| slice.to_vec())
        .collect();
        records[index] = new_content;
        let strings: String = records
        .iter()
        .map(|subvec| String::from_utf8_lossy(subvec).to_string())
        .collect::<Vec<_>>()
        .join("\n");
        if let Err(err) = FsApi::write(&(path.to_owned()+".tmp"), &strings) {
          return Err(err);
        }
        if let Err(err) = FsApi::rename(&(path.to_owned()+".tmp"), &path) {
          return Err(err);
        }
        Ok(())
      },
      Err(err) => Err(err)
    }
  }

  fn delete_content_in_file(path: &str, index: usize) -> Result<(),
  String> {
    match FsApi::read(path) {
      Ok(data) => {
        let bytes: Vec<u8> = data.as_bytes().to_vec();
        let mut records: Vec<Vec<u8>> = bytes.split(|&byte| byte == b'\n')
        .map(|slice| slice.to_vec())
        .collect();
        records.remove(index);
        let strings: String = records
        .iter()
        .map(|subvec| String::from_utf8_lossy(subvec).to_string())
        .collect::<Vec<_>>()
        .join("\n");
        if let Err(err) = FsApi::write(&(path.to_owned()+".tmp"), &strings) {
          return Err(err);
        }
        if let Err(err) = FsApi::rename(&(path.to_owned()+".tmp"), &path) {
          return Err(err);
        }
        Ok(())
      },
      Err(err) => Err(err)
    }
  }

  fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(),
  String> {
    fs::create_dir_all(&dst).map_err(|e| format!("Failed to create directory: {}", e))?;

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(src).map_err(|e| format!("Failed to read directory: {}", e))? {
      let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
      let metadata = entry.metadata().map_err(|e| format!("Failed to get metadata: {}", e))?;

      // Construct the destination path
      let dst_path = dst.as_ref().join(entry.file_name());

      // Check if the entry is a directory
      if metadata.is_dir() {
        // Recursively copy the directory
        Self::copy(entry.path(), &dst_path).map_err(|e| format!("Failed to copy directory: {}", e))?;
      } else {
        // Copy the file
        fs::copy(entry.path(), &dst_path).map_err(|e| format!("Failed to copy file: {}", e))?;
      }
    }
    Ok(())
  }
}