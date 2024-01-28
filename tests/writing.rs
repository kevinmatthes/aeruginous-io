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

mod path_buf_like_io {

    use aeruginous_io::{PathBufLikeReader, PathBufLikeTruncation};

    #[test]
    fn asset_manipulation_truncate() {
        let f = "path_buf_like_io_asset_manipulation_truncate.txt";

        assert!("tests/assets/GPL-3.0.rs"
            .read_silently()
            .map(|s| {
                s.lines()
                    .map(str::trim_start)
                    .filter(|l| (l.starts_with("///")))
                    .map(|l| {
                        if l.len() > 3 {
                            l.split_at(4).1.trim_end().to_string() + "\n"
                        } else {
                            "\n".to_string()
                        }
                    })
                    .collect::<String>()
            })
            .unwrap()
            .truncate_silently(f)
            .is_ok());

        assert_eq!(
            f.read_silently().unwrap(),
            "\
Copyright (C) 2024 Kevin Matthes

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"
        );

        std::fs::remove_file(f).unwrap();
    }
}

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

mod option_truncation {
    use aeruginous_io::{OptionTruncation, PathBufLikeReader};

    #[test]
    fn truncate_loudly_success_none() {
        let mut buffer = Vec::new();

        assert!("test"
            .truncate_loudly(None::<std::path::PathBuf>, &mut buffer)
            .is_ok());
        assert_eq!(buffer, b"test");
    }

    #[test]
    fn truncate_loudly_success_some() {
        let f = "option_truncation_truncate_loudly_success_some.txt";

        assert!("test\n".truncate_loudly(Some(f), &mut Vec::new()).is_ok());
        assert_eq!(f.read_silently().unwrap(), "test\n");

        std::fs::remove_file(f).unwrap();
    }

    #[test]
    fn truncate_silently_success_none() {
        let mut buffer = Vec::new();

        assert!("test"
            .truncate_silently(None::<std::path::PathBuf>, &mut buffer)
            .is_ok());
        assert_eq!(buffer, b"test");
    }

    #[test]
    fn truncate_silently_success_some() {
        let f = "option_truncation_truncate_silently_success_some.txt";

        assert!("test\n".truncate_silently(Some(f), &mut Vec::new()).is_ok());
        assert_eq!(f.read_silently().unwrap(), "test\n");

        std::fs::remove_file(f).unwrap();
    }
}

mod writer {
    use aeruginous_io::Writer;

    #[test]
    fn write_loudly_success() {
        let mut buffer = Vec::new();

        assert!("test".write_loudly(&mut buffer).is_ok());
        assert_eq!(buffer, b"test");
    }

    #[test]
    fn write_silently_success() {
        let mut buffer = Vec::new();

        assert!("test".write_silently(&mut buffer).is_ok());
        assert_eq!(buffer, b"test");
    }
}

/******************************************************************************/
