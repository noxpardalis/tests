#[cfg(test)]
mod tests {
    #[test]
    fn test_filetime_for_windows() {
        let path = std::env::temp_dir().join("test_mtime_zero.tmp");
        std::fs::File::create(&path).expect("failed to create temp file");

        let meta = std::fs::metadata(&path).expect("failed to read metadata");
        let mtime = filetime::FileTime::from_last_modification_time(&meta);

        filetime::set_file_mtime(&path, mtime).expect("failed to set mtime");

        std::fs::remove_file(&path).ok();
        assert_eq!(mtime, filetime::FileTime::zero());
    }
}
