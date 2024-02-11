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

mod buf_read_reader {
    use aeruginous_io::BufReadReader;

    #[test]
    fn read_loudly_failure() {
        assert!("👍"
            .as_bytes()
            .iter()
            .map(|b| if b > &200 { 0 } else { *b })
            .collect::<Vec<u8>>()
            .read_loudly()
            .is_err());
    }

    #[test]
    fn read_loudly_success() {
        assert_eq!({ &(b"test"[..]) }.read_loudly().unwrap(), "test\n");
    }

    #[test]
    fn read_silently_failure() {
        assert!("👍"
            .as_bytes()
            .iter()
            .map(|b| if b > &200 { 0 } else { *b })
            .collect::<Vec<u8>>()
            .read_silently()
            .is_err());
    }

    #[test]
    fn read_silently_success() {
        assert_eq!({ &(b"test"[..]) }.read_silently().unwrap(), "test\n");
    }
}

mod option_reader {
    use aeruginous_io::OptionReader;

    #[test]
    fn method_result_equality_none() {
        assert_eq!(
            None::<&str>.read_loudly(&b"test"[..]).unwrap(),
            None::<&str>.read_silently(&b"test"[..]).unwrap()
        );
    }

    #[test]
    fn method_result_equality_some() {
        assert_eq!(
            Some("tests/assets/GPL-3.0.rs")
                .read_loudly(&b""[..])
                .unwrap(),
            Some("tests/assets/GPL-3.0.rs")
                .read_silently(&b""[..])
                .unwrap(),
        );
    }

    #[test]
    fn read_silently_success_none() {
        assert_eq!(None::<&str>.read_silently(&b"test"[..]).unwrap(), "test\n");
    }

    #[test]
    fn read_silently_success_some() {
        assert_eq!(
            Some("tests/assets/GPL-3.0.rs")
                .read_silently(&b""[..])
                .unwrap(),
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
}

mod path_buf_like_reader {
    use aeruginous_io::PathBufLikeReader;

    #[test]
    fn method_result_equality() {
        assert_eq!(
            "tests/assets/GPL-3.0.rs".read_loudly().unwrap(),
            "tests/assets/GPL-3.0.rs".read_silently().unwrap(),
        );
    }

    #[test]
    fn read_loudly_failure_attempt_to_read_directory() {
        assert!(".github/".read_loudly().is_err());
    }

    #[test]
    fn read_loudly_failure_file_does_not_exist() {
        assert!("no_such_file.txt".read_loudly().is_err());
    }

    #[test]
    fn read_silently_failure_attempt_to_read_directory() {
        assert!(".github/".read_silently().is_err());
    }

    #[test]
    fn read_silently_failure_file_does_not_exist() {
        assert!("no_such_file.txt".read_silently().is_err());
    }

    #[test]
    fn read_silently_success() {
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
}

mod vector_reader {
    use aeruginous_io::VectorReader;

    #[test]
    fn method_result_equality_empty() {
        assert_eq!(
            Vec::<&str>::new().read_loudly(&b"test"[..]).unwrap(),
            Vec::<&str>::new().read_silently(&b"test"[..]).unwrap(),
        );
    }

    #[test]
    fn method_result_equality_non_empty() {
        assert_eq!(
            vec!["tests/assets/GPL-3.0.rs", "tests/assets/GPL-3.0.rs"]
                .read_loudly(&b""[..])
                .unwrap(),
            vec!["tests/assets/GPL-3.0.rs", "tests/assets/GPL-3.0.rs"]
                .read_silently(&b""[..])
                .unwrap(),
        );
    }

    #[test]
    fn read_silently_success_empty() {
        assert_eq!(
            Vec::<&str>::new().read_silently(&b"test"[..]).unwrap(),
            "test\n"
        );
    }

    #[test]
    fn read_silently_success_non_empty() {
        assert_eq!(
            vec!["tests/assets/GPL-3.0.rs", "tests/assets/GPL-3.0.rs"]
                .read_silently(&b""[..])
                .unwrap(),
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
}

/******************************************************************************/
