use std::env;

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{all_consuming, fail, map_res},
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
    u8::from_str_radix(input, 16).and_then(|n| Ok(n * 16 + n))
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

// NOTE: my attempt at the existing 'all_consuming' combinator before I knew it existed
fn all_done(input: &str) -> IResult<&str, ()> {
    if input.len() == 0 {
        Ok((input, ()))
    } else {
        // NOTE: ErrorKind::Fail error used rather then ErrorKind::Eof returned by 'all_consuming'
        fail(input)
    }
}

pub fn hex_color3(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary1, hex_primary1, hex_primary1))(input)?;
    let (input, _) = all_done(input)?;

    Ok((input, Color { red, green, blue }))
}

// NOTE: 'all_consuming' used as 'eof' detector
pub fn hex_color6(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) =
        all_consuming(tuple((hex_primary2, hex_primary2, hex_primary2)))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    alt((hex_color6, hex_color3))(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Usage: Give a list of hex colours - eg '#111' '#BEEFCE'\n");
    }

    args.iter().enumerate().skip(1).for_each(|a| {
        const HEX_LEAD: char = '#';
        let mut input = a.1.clone();

        if !input.starts_with(HEX_LEAD) {
            input.insert(0, HEX_LEAD);
        }

        let res = hex_color(&input);

        match res {
            Ok(col) => print!("{} => {:?}\n", input, col.1),
            Err(_err) => print!("ERROR: {} is not a valid colour!\n", input),
        }
    });
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
