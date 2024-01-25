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


use std::{io::Write, path::PathBuf};
use sysexits::Result;

/// Truncate files given as instances convertible to a [`std::path::PathBuf`].
pub trait PathBufLikeTruncation<T>
where
    PathBuf: From<T>,
{
    /// Truncate the given file using the data this method is called on.
    ///
    /// This method behaves just like
    /// [`crate::PathBufLikeTruncation::truncate_silently`] despite also
    /// printing error messages to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn truncate_loudly(self, destination: T) -> Result<()>;

    /// Truncate the given file using the data this method is called on.
    ///
    /// The data this method is called on will be converted to a [`String`] and
    /// written to the given file.  The data therefore needs to implement
    /// [`ToString`], the file needs to be convertible to a
    /// [`std::path::PathBuf`].  The file will be truncated.
    ///
    /// The return value is either the unit type, in case of success, or a
    /// [`sysexits::ExitCode`] to describe the error cause, otherwise.
    ///
    /// Error messages are not written to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn truncate_silently(self, destination: T) -> Result<()>;
}

impl<P, T: ToString> PathBufLikeTruncation<P> for T
where
    PathBuf: From<P>,
{
    fn truncate_loudly(self, destination: P) -> Result<()> {
        match std::fs::File::options()
            .append(false)
            .create(true)
            .truncate(true)
            .write(true)
            .open(PathBuf::from(destination))
        {
            Err(e) => {
                eprintln!("{e}");
                Err(e.into())
            }
            Ok(mut file) => {
                let bytes = self.to_string().as_bytes().to_vec();

                match file.write(&bytes) {
                    Err(e) => {
                        eprintln!("{e}");
                        Err(e.into())
                    }
                    Ok(n) => {
                        if n == bytes.len() {
                            Ok(())
                        } else {
                            eprintln!(
                                "Creating an exact copy was not possible."
                            );
                            Err(sysexits::ExitCode::IoErr)
                        }
                    }
                }
            }
        }
    }

    fn truncate_silently(self, destination: P) -> Result<()> {
        match std::fs::File::options()
            .append(false)
            .create(true)
            .truncate(true)
            .write(true)
            .open(PathBuf::from(destination))
        {
            Err(e) => Err(e.into()),
            Ok(mut file) => {
                let bytes = self.to_string().as_bytes().to_vec();

                match file.write(&bytes) {
                    Err(e) => Err(e.into()),
                    Ok(n) => {
                        if n == bytes.len() {
                            Ok(())
                        } else {
                            Err(sysexits::ExitCode::IoErr)
                        }
                    }
                }
            }
        }
    }
}

/******************************************************************************/
