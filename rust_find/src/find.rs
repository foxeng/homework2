use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

/// An abstracted view of a file
#[derive(Clone)]
pub struct File {
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    // TODO OPT: parent dir / filename?
    // TODO OPT: permissions? owner?
}

impl File {
    /// Construct a File from a path
    pub fn from<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref();
        let metadata = path.metadata()?;
        Ok(File {
            path: String::from(path.to_string_lossy()),
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        })
    }
}

/// Traverse the file tree rooted at file, returning the files (and directories)
/// with paths matching pats and size greater than size bytes
pub fn traverse(file: &File, pats: &Vec<String>, size: u64) -> Vec<File> {
    let res = pats
        .iter()
        .map(|p| Regex::new(p).unwrap_or_else(|_| Regex::new("").unwrap()))
        .collect();
    do_traverse(file, &res, size)
}

fn do_traverse(file: &File, res: &Vec<regex::Regex>, size: u64) -> Vec<File> {
    // TODO OPT: Return iterator instead of vec
    let mut v = vec![];
    // TODO OPT: Use closures instead
    if filter_path(file, res) && filter_size(file, size) {
        v.push(file.clone());
    }

    if !file.is_dir {
        return v;
    }
    // Descend
    let d = match fs::read_dir(&file.path) {
        Err(e) => {
            eprintln!("read dir {}: {}", file.path, e);
            return v;
        }
        Ok(d) => d,
    };
    for f in d {
        let f = match f {
            Err(e) => {
                eprintln!("read dir {}: {}", file.path, e);
                continue;
            }
            Ok(f) => f,
        };
        let subfile = match File::from(f.path()) {
            Err(e) => {
                eprintln!("get metadata for {}: {}", f.path().to_string_lossy(), e);
                continue;
            }
            Ok(subfile) => subfile,
        };
        v.extend(do_traverse(&subfile, res, size))
    }

    v
}

fn filter_path(file: &File, res: &Vec<regex::Regex>) -> bool {
    for re in res {
        if !re.is_match(&file.path) {
            return false;
        }
    }
    true
}

#[test]
fn test_filter_path() {
    let file = File {
        path: String::from("/foo/bar.txt"),
        size: 0,
        is_dir: false,
    };

    let re = Regex::new(r"baz").unwrap();
    assert!(!filter_path(&file, &vec![re]));
    let re = Regex::new(r"bar").unwrap();
    assert!(filter_path(&file, &vec![re]));
    let re = Regex::new(r".*.txt").unwrap();
    assert!(filter_path(&file, &vec![re]));
}

fn filter_size(file: &File, size: u64) -> bool {
    file.size > size
}

#[test]
fn test_filter_size() {
    let file = File {
        path: String::from(""),
        size: 10,
        is_dir: false,
    };

    assert!(!filter_size(&file, 11));
    assert!(filter_size(&file, 1))
}
