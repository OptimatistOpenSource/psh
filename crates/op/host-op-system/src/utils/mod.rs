use std::{
    env,
    path::{Path, PathBuf},
};

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
    fn test_which() {
        use super::which;
        println!("{:?}", which("ls").unwrap());
    }
}
