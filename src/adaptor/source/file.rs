#![cfg(feature = "file")]
use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use super::{Source, SourceType};
use crate::{
    Parser, RealmeError, Value,
    realme::watcher::{Channel, Event},
};

/// Represents a source that reads configuration data from a file.
///
/// This struct is generic over `T` which is the parser type used to parse the
/// file contents, and `U` which is the type of the path (defaults to
/// `PathBuf`).
///
/// # Type Parameters
///
/// * `T`: The parser type that implements the `Parser` trait for parsing file
///   contents.
/// * `U`: The path type that implements `AsRef<Path>`, defaults to `PathBuf`.
#[derive(Debug)]
pub struct FileSource<T, U = PathBuf> {
    /// The path to the configuration file.
    path: U,
    /// Phantom data to hold the parser type.
    _marker: PhantomData<T>,
}

impl<U: AsRef<Path>, T> FileSource<T, U> {
    /// Constructs a new `FileSource` with the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the file that will be read.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::path::PathBuf;
    ///
    /// use realme::{FileSource, TomlParser};
    ///
    /// let file_source = FileSource::<TomlParser>::new(PathBuf::from(
    ///     "path/to/your/config.toml",
    /// ));
    /// ```
    pub const fn new(path: U) -> Self {
        Self {
            path,
            _marker: PhantomData,
        }
    }
}

impl<T, U> Source for FileSource<T, U>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
    U: AsRef<Path> + Send + Sync,
{
    /// Parses the file at the specified path using the parser type `T`.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - If the file is successfully read and parsed.
    /// * `Err(RealmeError)` - If there is an error reading the file or parsing
    ///   its contents.
    ///
    /// # Errors
    ///
    /// This method returns `Err(RealmeError)` if the file cannot be read or if
    /// the parsing fails.
    fn parse(&self) -> Result<Value, RealmeError> {
        let buffer = std::fs::read_to_string(self.path.as_ref())
            .map_err(|e| RealmeError::ReadFileError(e.to_string()))?;
        let parsed = T::parse(&buffer).map_err(|e| {
            RealmeError::new_parse_error(
                self.path.as_ref().to_string_lossy().to_string(),
                e.to_string(),
            )
        })?;

        Value::try_serialize(&parsed)
            .map_err(|e| RealmeError::BuildError(e.to_string()))
    }

    /// Returns the source type of this `FileSource`.
    ///
    /// # Returns
    ///
    /// Always returns `SourceType::Str`.
    fn source_type(&self) -> SourceType {
        SourceType::Str
    }

    fn watch(&self, chan: Channel, interval: Duration) {
        thread::spawn(move || match chan.1.recv() {
            Ok(Event::Stopped) => (),
            Ok(e) => {
                RealmeError::Unknown(format!("{e:?}"));
            }
            Err(e) => {
                RealmeError::Unknown(format!("{e:?}"));
            }
        });
        let path = self.path.as_ref().to_path_buf();
        thread::spawn(move || {
            let mut last_modified =
                fs::metadata(&path).unwrap().modified().unwrap();
            loop {
                thread::sleep(interval);
                let modified = fs::metadata(&path).unwrap().modified().unwrap();
                if modified != last_modified {
                    last_modified = modified;
                    chan.0.send(Event::Changed).unwrap();
                }
            }
        });
    }
}
