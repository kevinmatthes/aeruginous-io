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

mod path_buf_like_truncation {
    use aeruginous_io::{PathBufLikeReader, PathBufLikeTruncation};

    #[test]
    fn truncate_loudly_failure() {
        assert!(String::new().truncate_loudly("tests/").is_err());
    }

    #[test]
    fn truncate_loudly_success() {
        let f = "path_buf_like_truncation_truncate_loudly_success.txt";

        assert!("test\n".truncate_loudly(f).is_ok());
        assert_eq!(f.read_silently().unwrap(), "test\n");

        std::fs::remove_file(f).unwrap();
    }

    #[test]
    fn truncate_silently_failure() {
        assert!(String::new().truncate_silently("tests/").is_err());
    }

    #[test]
    fn truncate_silently_success() {
        let f = "path_buf_like_truncation_truncate_silently_success.txt";

        assert!("test\n".truncate_silently(f).is_ok());
        assert_eq!(f.read_silently().unwrap(), "test\n");

        std::fs::remove_file(f).unwrap();
    }
}

/******************************************************************************/
