use std::path::{Path,PathBuf};

/// Get the path to the current executable file
pub fn get_current_dir() -> std::io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();
    
    if path.is_dir() {
        return Ok(path);
    }
    Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, format!("{}",path.to_str().expect("path to str failed"))))
}

/// Find the path of the executable file with the specified name in the PATH environment variable path
pub fn find_process(name: &str) -> String {
    if let Ok(mut path) = get_current_dir() {
        path.push(name);
        
        if path.is_file() {
            return path.to_str().expect("is not normal path").to_string();
        }
    }

    return find_process_from_path(name);
}

/// Get the full path of the specified program name
pub fn find_process_from_path(name: &str) -> String {
    let key = "PATH";
    if let Ok(val) = std::env::var(key) {
        let paths :Vec<&str>= val.split(':').collect();

        for path in paths {
            let mut filepath = Path::new(path).to_path_buf();
            filepath.push(name);

            if filepath.is_file() {
                return filepath.to_str().expect("is not normal path").to_string();
            }
        }
    }

    return "".to_string();
}

/// Get all files with a specific suffix under the specified path
pub fn get_files(path: PathBuf,suffix: &str) -> Vec<String> {
    let mut result = Vec::new();
    for entry in path.read_dir().expect("this is not dir") {
        let entry = entry.expect("this is entry...");
        if let Some(x) = entry.path().extension() {
            if x == suffix {
                result.push(entry.file_name().to_str().expect("is normal string").to_string());
            }
        }
    }

    return result;
}

/// Get the user configuration directory
pub fn user_config_dir() ->std::io::Result<PathBuf> {
    if cfg!(target_os = "windows") {
        match std::env::var("APPDATA") {
            Ok(val) => {
                Ok(PathBuf::from(val))
            },
            Err(_) => {
                match std::env::var("HOMEPATH") {
                    Ok(val) => {
                        let mut path = PathBuf::from(val);
                        path.push("Application Data");
                        Ok(path)
                    },
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::NotFound,e)),
                }
            },
        }
    } else {
        match std::env::var("XDG_CONFIG_HOME") {
            Ok(val) => {
                Ok(PathBuf::from(val))
            },
            Err(_) => {
                match std::env::var("HOME") {
                    Ok(val) => {
                        let mut path = PathBuf::from(val);
                        path.push(".config");
                        Ok(path)
                    },
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::NotFound,e)),
                }
            },
        }
    }
    
}
