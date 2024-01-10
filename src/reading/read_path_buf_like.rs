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

/// Read from files given as instances convertible to a [`std::path::PathBuf`].
pub trait ReadPathBufLike {
    /// Read from the file this method is called on.
    ///
    /// The instance this method is called on needs to be convertible to a
    /// [`std::path::PathBuf`].  The return value either contains the read file
    /// contents as a [`String`] in case of success or a [`sysexits::ExitCode`]
    /// to describe the error cause, otherwise.  Error messages are not written
    /// to [`std::io::Stderr`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn read_silently(&self) -> Result<String>;

    /// Read from the file this method is called on.
    ///
    /// This method behaves just like [`crate::ReadPathBufLike::read_silently`]
    /// but also prints error messages to [`std::io::Stderr`].
    fn read_loudly(&self) -> Result<String>;
}

impl<T> ReadPathBufLike for T
where
    PathBuf: From<T>,
    T: Clone,
{
    fn read_silently(&self) -> Result<String> {
        match std::fs::read_to_string(PathBuf::from(self.clone())) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    fn read_loudly(&self) -> Result<String> {
        match std::fs::read_to_string(PathBuf::from(self.clone())) {
            Ok(s) => Ok(s),
            Err(e) => {
                eprintln!("{e}");
                Err(e.into())
            }
        }
    }
}

/******************************************************************************/
