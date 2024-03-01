#![allow(dead_code)]
use nix::unistd::geteuid;

fn check_root_privilege() -> bool {
    let euid = geteuid();
    euid.is_root()
}

mod tests {
    #[test]
    fn test_check_root_privilege() {
        // Test when the user has root privilege
        use crate::op::common::utils::check_root_privilege;
        assert_eq!(check_root_privilege(), true);

        // Test when the user does not have root privilege
        // You can modify this test case to simulate a non-root user
        // by returning a non-root euid from geteuid() function
        // assert_eq!(check_root_privilege(), false);
    }
}
