use std::ffi::OsStr;
use std::io::Error;
use std::process::Command;

/// Run program with args
///
/// Example:
///
/// ```rust
/// let bytes = run_program("echo", ["hello world"]).unwrap();
/// assert_eq!("hello world\n", String::from_utf8(bytes).unwrap());
/// ```
pub fn run_program<P, A>(program: P, args: A) -> Result<Vec<u8>, Error>
where
    P: AsRef<OsStr>,
    A: IntoIterator<Item = P>,
{
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.output().map(|output| output.stdout)
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn test_run_program() {
        let output = run_program("echo", ["hello world"]).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!("hello world\n", output_str);
    }
}
