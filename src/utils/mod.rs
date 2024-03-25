use std::{
    env,
    path::{Path, PathBuf},
};

use nix::unistd::geteuid;

#[allow(dead_code)]
fn check_root_privilege() -> bool {
    let euid = geteuid();
    euid.is_root()
}

#[allow(dead_code)]
pub(crate) fn which<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_check_root_privilege() {
        use super::check_root_privilege;
        // Test when the user has root privilege
        assert_eq!(check_root_privilege(), true);

        // Test when the user does not have root privilege
        // You can modify this test case to simulate a non-root user
        // by returning a non-root euid from geteuid() function
        // assert_eq!(check_root_privilege(), false);
    }

    #[test]
    fn test_which() {
        use super::which;
        println!("{:?}", which("ls").unwrap());
    }
}
