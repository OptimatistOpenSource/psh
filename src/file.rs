use std::fs::{create_dir_all, File};
use std::io::Write;
use std::ops::Not;
use std::path::Path;

/// Write bytes to file with `File::write_all`
///
/// This will create a directory and all of its parent components if they are missing
///
/// Example:
/// ```rust
/// let path = Path::new("foo/bar/baz");
/// let bytes = "hello world".as_bytes();
/// write_file(path, bytes).unwrap();
/// assert_eq!(fs::read(path).unwrap(), bytes);
/// ```
pub fn write_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    match path.parent() {
        Some(parent) if parent.exists().not() => {
            create_dir_all(parent)?;
        }
        _ => {}
    }

    let mut file = File::create(path)?;
    file.write_all(bytes)
}

#[cfg(test)]
mod tests {
    use super::write_file;
    use std::fs;
    use std::fs::{remove_file, File};
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_write_file() {
        let rand_num = {
            let mut urandom = File::open("/dev/urandom").unwrap();
            let mut buf = [0u8; 16];
            urandom.read_exact(&mut buf).unwrap();
            buf.iter().fold(0_usize, |acc, i| acc + *i as usize)
        };

        let rand_path = format!("/tmp/foo-{}/bar/baz", rand_num);
        let rand_path = Path::new(&rand_path);

        let bytes = "hello world".as_bytes();
        write_file(rand_path, bytes).unwrap();
        assert_eq!(fs::read(rand_path).unwrap(), bytes);

        remove_file(rand_path).unwrap();
    }
}
