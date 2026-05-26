#[derive(Debug, Clone)]
pub struct Config {
    // ----- Security-related options -----
    /// Bypasses internal checks for directory traversal and uses the path as-is.
    /// This is a huge security risk and should only be used if you really trust the input.
    ///
    /// **Default:** `false`
    ///
    /// If this is false, the path gets normalized and will never be able to escape the extraction root.
    /// Examples of how paths get normalized when this is false:
    /// - `file.txt` -> `archive/file.txt`
    /// - `../file.txt` -> `archive/file.txt`
    /// - `../../../../file.txt` -> `archive/file.txt`
    /// - `a/../b/file.txt` -> `archive/b/file.txt`
    /// - `/usr/bin/bash` -> `archive/usr/bin/bash`
    pub security_allow_directory_traversal: bool,

    /// Warns the user if an archive attempts directory traversal.
    ///
    /// **Default:** `true`
    ///
    /// This gets turned off automatically if `security_allow_directory_traversal` is true,
    /// since this flag skips all checks for directory traversal and doesn't sanitize the output path.
    pub security_warn_directory_traversal: bool,

    /// Aborts execution if running as root.
    ///
    /// **Default:** `true`
    ///
    /// This mainly prevents the user from overriding system files by accident.
    pub security_deny_sudo: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            security_allow_directory_traversal: false,
            security_warn_directory_traversal: true,
            security_deny_sudo: true,
        }
    }
}
