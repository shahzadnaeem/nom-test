use std::env;

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex1(input: &str) -> Result<u8, std::num::ParseIntError> {
    match u8::from_str_radix(input, 16) {
        Ok(num) => Ok(num + 16 * num),
        err => err,
    }
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary1(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 1, is_hex_digit), from_hex1)(input)
}

fn hex_primary2(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color6(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary2, hex_primary2, hex_primary2))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn hex_color3(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary1, hex_primary1, hex_primary1))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    alt((hex_color6, hex_color3))(input)
}

fn main() {
    // const INPUT: &str = "#555555";
    // let res = hex_color(INPUT);

    // match res {
    //     Ok(col) => print!("{} => {:?}\n", INPUT, col.1),
    //     Err(err) => print!("ERROR: {:?}", err),
    // };

    // const INPUT2: &str = "#FFF";
    // let res = hex_color(INPUT2);

    // match res {
    //     Ok(col) => print!("{} => {:?}\n", INPUT2, col.1),
    //     Err(err) => print!("ERROR: {:?}", err),
    // };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Usage: Give a list of hex colours - eg '#111' '#BEEFCE'\n");
    }

    args.iter()
        .enumerate()
        .filter(|&(i, _)| i != 0)
        .for_each(|a| {
            let res = hex_color(a.1);

            match res {
                Ok(col) => print!("{} => {:?}\n", a.1, col.1),
                Err(_err) => print!("ERROR: {} is not a valid colour!\n", a.1),
            }
        });
}

#[test]
fn parse_color() {
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
