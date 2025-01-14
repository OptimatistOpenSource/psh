pub fn dev_speed(dev: &str) -> Option<u32> {
    let Ok(speed) = std::fs::read_to_string(format!("/sys/class/net/{dev}/speed")) else {
        None?
    };

    parse_speed(speed.trim())
}

/// return Mb/s
fn parse_speed(speed: &str) -> Option<u32> {
    speed.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let s = parse_speed("-1");
        assert_eq!(s, None);
        let s = parse_speed("");
        assert_eq!(s, None);
        let s = parse_speed("1000");
        assert_eq!(s, Some(1000));
        let s = parse_speed("400000");
        assert_eq!(s, Some(400000));
    }
}
