//! Platform dependent types.

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::Cow;
        use std::fmt;
        use std::path::PathBuf;
        use std::prelude::v1::*;
        use std::str;
    } else if #[cfg(target_os = "theseus")] {
        use alloc::borrow::Cow;
        use alloc::string::String;
        use core::fmt;
        use theseus_path_std::PathBuf;
        use core::str;
    }
}

/// A platform independent representation of a string. When working with `std`
/// enabled it is recommended to the convenience methods for providing
/// conversions to `std` types.
#[derive(Debug)]
pub enum BytesOrWideString<'a> {
    /// A slice, typically provided on Unix platforms.
    Bytes(&'a [u8]),
    /// Wide strings typically from Windows.
    Wide(&'a [u16]),
}

#[cfg(any(feature = "std", target_os = "theseus"))]
impl<'a> BytesOrWideString<'a> {
    /// Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid
    /// UTF-8 or if `BytesOrWideString` is `Wide`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn to_str_lossy(&self) -> Cow<'a, str> {
        use self::BytesOrWideString::*;

        match self {
            &Bytes(slice) => String::from_utf8_lossy(slice),
            &Wide(wide) => Cow::Owned(String::from_utf16_lossy(wide)),
        }
    }

    /// Provides a `Path` representation of `BytesOrWideString`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn into_path_buf(self) -> PathBuf {
        #[cfg(unix)]
        {
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            if let BytesOrWideString::Bytes(slice) = self {
                return PathBuf::from(OsStr::from_bytes(slice));
            }
        }

        #[cfg(windows)]
        {
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

            if let BytesOrWideString::Wide(slice) = self {
                return PathBuf::from(OsString::from_wide(slice));
            }
        }

        if let BytesOrWideString::Bytes(b) = self {
            if let Ok(s) = str::from_utf8(b) {
                return PathBuf::from(s);
            }
        }
        unreachable!()
    }
}

#[cfg(any(feature = "std", target_os = "theseus"))]
impl<'a> fmt::Display for BytesOrWideString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_str_lossy().fmt(f)
    }
}
