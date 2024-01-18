/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2024 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

use std::path::PathBuf;
use sysexits::Result;

/// Read from a [`std::io::BufRead`]er.
pub trait BufReadReader {
    /// Read from a [`std::io::BufRead`]er and print error messages.
    ///
    /// This method behaves just like
    /// [`crate::BufReadReader::read_silently`] despite also printing error
    /// messages to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_loudly(self) -> Result<String>;

    /// Read from a [`std::io::BufRead`]er and suppress any error messages.
    ///
    /// The instance this method is called on needs to implement
    /// [`std::io::BufRead`] and will be consumed in order to retrieve its
    /// content.  The instance will be read line by line, subsequent lines are
    /// joined by a single newline character (`\n`).  The last line will have a
    /// trailing newline character.
    ///
    /// The return value is either the read content as a [`String`], in case of
    /// success, or a [`sysexits::ExitCode`] to describe the error cause,
    /// otherwise.
    ///
    /// Error messages are not written to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_silently(self) -> Result<String>;
}

impl<T: std::io::BufRead> BufReadReader for T {
    fn read_loudly(self) -> Result<String> {
        let mut result = String::new();

        for line in self.lines() {
            match line {
                Ok(l) => {
                    result.push_str(&l);
                    result.push('\n');
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{e}");
                    Err(e)
                }
            }?;
        }

        Ok(result)
    }

    fn read_silently(self) -> Result<String> {
        let mut result = String::new();

        for line in self.lines() {
            result.push_str(&line?);
            result.push('\n');
        }

        Ok(result)
    }
}

/// Read from either source, dependending on the circumstances.
pub trait OptionReader<B>
where
    B: BufReadReader,
{
    /// Read from this instance or the given alternative.
    ///
    /// This method behaves just like
    /// [`crate::OptionReader::read_silently`] despite also printing error
    /// messages to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_loudly(&self, alternative: B) -> Result<String>;

    /// Read from this instance or the given alternative.
    ///
    /// If the instance this method is called has [`Some`] value, the contained
    /// value will be interpreted as file to read from.  Therefore, the value
    /// needs to implement [`crate::PathBufLikeReader`].  In case the instance
    /// this method is called on is [`None`], the given alternative will be
    /// considered the source to read from.  Therefore, the alternative needs to
    /// implement [`crate::BufReadReader`].
    ///
    /// The return value is either the read content as a [`String`], in case of
    /// success, or a [`sysexits::ExitCode`] to describe the error cause,
    /// otherwise.
    ///
    /// Error messages are not written to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_silently(&self, alternative: B) -> Result<String>;
}

impl<B: std::io::BufRead, P: PathBufLikeReader> OptionReader<B> for Option<P> {
    fn read_loudly(&self, alternative: B) -> Result<String> {
        self.as_ref().map_or_else(
            || alternative.read_loudly(),
            PathBufLikeReader::read_loudly,
        )
    }

    fn read_silently(&self, alternative: B) -> Result<String> {
        self.as_ref().map_or_else(
            || alternative.read_silently(),
            PathBufLikeReader::read_silently,
        )
    }
}

/// Read from files given as instances convertible to a [`std::path::PathBuf`].
pub trait PathBufLikeReader {
    /// Read from the file this method is called on.
    ///
    /// This method behaves just like
    /// [`crate::PathBufLikeReader::read_silently`] despite also printing error
    /// messages to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_loudly(&self) -> Result<String>;

    /// Read from the file this method is called on.
    ///
    /// The instance this method is called on needs to be convertible to a
    /// [`std::path::PathBuf`].  The referenced file will be opened and read.
    ///
    /// The return value is either the read content as a [`String`], in case of
    /// success, or a [`sysexits::ExitCode`] to describe the error cause,
    /// otherwise.
    ///
    /// Error messages are not written to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_silently(&self) -> Result<String>;
}

impl<T> PathBufLikeReader for T
where
    PathBuf: From<T>,
    T: Clone,
{
    fn read_loudly(&self) -> Result<String> {
        match std::fs::read_to_string(PathBuf::from(self.clone())) {
            Ok(s) => Ok(s),
            Err(e) => {
                eprintln!("{e}");
                Err(e.into())
            }
        }
    }

    fn read_silently(&self) -> Result<String> {
        match std::fs::read_to_string(PathBuf::from(self.clone())) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }
}

/******************************************************************************/
