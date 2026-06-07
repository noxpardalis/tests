#[cfg(test)]
mod tests {
    const TIME_NEVER: i64 = 9223372036854775807;

    #[test]
    fn test_duration_01() {
        let standard =
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_nanos(TIME_NEVER as u64);
        let value = standard.duration_since(std::time::UNIX_EPOCH).unwrap();

        panic!(
            "{value:?} {} {TIME_NEVER} {:?} {:?}",
            value.as_nanos(),
            std::time::SystemTime::UNIX_EPOCH,
            std::time::SystemTime::UNIX_EPOCH.duration_since(std::time::SystemTime::UNIX_EPOCH)
        );
    }

    #[test]
    fn test_duration_02() {
        let standard = std::time::SystemTime::UNIX_EPOCH - std::time::Duration::from_nanos(1);
        let value01 = standard
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();

        let nanos = u64::MAX;
        let standard = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_nanos(nanos);
        let value02 = standard
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        panic!(
            "{value01:?} {}\n{value02:?}: {} {:?} {:?}",
            value01.as_nanos(),
            value02.as_nanos(),
            std::time::SystemTime::UNIX_EPOCH,
            std::time::SystemTime::UNIX_EPOCH.duration_since(std::time::SystemTime::UNIX_EPOCH)
        );
    }
}
