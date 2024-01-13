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

use aeruginous_io::{BufReadReader, PathBufLikeReader};

#[test]
fn read_buf_reader_failure() {
    assert!("👍"
        .as_bytes()
        .iter()
        .map(|b| if b > &200 { 0 } else { *b })
        .collect::<Vec<u8>>()
        .read_loudly()
        .is_err());
    assert!("👍"
        .as_bytes()
        .iter()
        .map(|b| if b > &200 { 0 } else { *b })
        .collect::<Vec<u8>>()
        .read_silently()
        .is_err());
}

#[test]
fn read_buf_read_success() {
    assert_eq!({ &(b"test"[..]) }.read_loudly().unwrap(), "test\n");
    assert_eq!({ &(b"test"[..]) }.read_silently().unwrap(), "test\n");
}

#[test]
fn read_path_buf_like_failure_directory() {
    assert!(".github/".read_loudly().is_err());
    assert!(".github/".read_silently().is_err());
}

#[test]
fn read_path_buf_like_failure_does_not_exist() {
    assert!("no_such_file.txt".read_loudly().is_err());
    assert!("no_such_file.txt".read_silently().is_err());
}

#[test]
fn read_path_buf_like_success() {
    assert_eq!(
        "tests/assets/GPL-3.0.rs".read_loudly().unwrap(),
        "tests/assets/GPL-3.0.rs".read_silently().unwrap(),
    );

    assert_eq!(
        "tests/assets/GPL-3.0.rs".read_silently().unwrap(),
        "\
/// Copyright (C) 2024 Kevin Matthes
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
"
    );
}

/******************************************************************************/
