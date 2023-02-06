use nom::error::ErrorKind;

use super::*;

#[test]
fn parse_color_6_digits() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}

#[test]
fn parse_color_3_digits() {
    assert_eq!(
        hex_color("#FFF"),
        Ok((
            "",
            Color {
                red: 255,
                green: 255,
                blue: 255,
            }
        ))
    );
}

#[test]
fn parse_color_4_digits_error() {
    assert_eq!(
        hex_color("#FFF1"),
        Err(nom::Err::Error(nom::error::Error {
            input: "1",
            code: ErrorKind::Fail
        }))
    );
}

#[test]
fn parse_color_7_digits_error() {
    assert_eq!(
        hex_color("#FFF1112"),
        Err(nom::Err::Error(nom::error::Error {
            input: "1112",
            code: ErrorKind::Fail
        }))
    );
}

#[test]
fn parse_hex_color6_7_digits_error() {
    assert_eq!(
        hex_color6("#FFF1112"),
        Err(nom::Err::Error(nom::error::Error {
            input: "2",
            code: ErrorKind::Eof
        }))
    );
}
