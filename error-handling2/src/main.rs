#![allow(dead_code)]

use std::{collections::HashMap, io, num::ParseIntError, error::Error, fmt::Display};

#[derive(Debug)]
struct Expiration {
    month: u32,
    year: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    expiration: Expiration,
    cvv: u32
}

use thiserror::Error;

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// impl std::fmt::Debug for CreditCardError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CreditCardError::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
//             CreditCardError::Other(e, msg) => write!(f, "{self}\n{msg}\n\t{e:?}")
//         }
//     }
// }

// impl Error for CreditCardError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             CreditCardError::InvalidInput(_) => None,
//             CreditCardError::Other(e, _) => Some(e.as_ref())
//         }
//     }
// }

// impl Display for CreditCardError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Credit card Error: Can not retrieve data")
//     }
// }

use anyhow::Context;
// errot type to return more detail error
// debug trait for developer

#[derive(Error, Debug)]    
#[error("error")]
struct ParsePaymentInfoError {
    // trait object => source should implements Error
    source: Option<anyhow::Error>,
    msg: Option<String>
}

// impl From<ParseIntError> for ParsePaymentInfoError{
//     fn from(e: ParseIntError) -> Self {
//         ParsePaymentInfoError {
//             // source is an option
//             source: Some(Box::new(e)),
//             msg: None
//         }
//     }
// }

// impl std::fmt::Debug for ParsePaymentInfoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(msg) = &self.msg {
//             write!(f, "{self}\n{msg}")?;
//         }

//         if let Some(e) = self.source.as_ref() {
//             write!(f, "\ncaused by:\n\t{e:?}")?;
//         }

//         Ok(())
//     }
// }

// impl Error for ParsePaymentInfoError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source.as_ref().map(|e| e.as_ref())
//     }
// }

// // for user 
// impl Display for ParsePaymentInfoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Parsing Payment Error: Invalid payment info")
//     }
// }

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| { 
            s.parse().with_context(|| format!("{s} could not be parsed to u32"))})
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| {
            ParsePaymentInfoError {
                source: Some(e),
                msg: Some(format!("Failed to parse {card} numbers."))
            }
        })?;
    Ok(numbers)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    // converted parseInt error to string
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError{ source: None, msg: Some(format!("Expected {expected_len} numbers but got {len}."))});
    }

    let cvv = numbers.pop().ok_or(ParsePaymentInfoError{source: None, msg: Some("Invalid card".to_owned())})?;
    let year = numbers.pop().ok_or(ParsePaymentInfoError{source: None, msg: Some("Invalid card".to_owned())})?;
    let month = numbers.pop().ok_or(ParsePaymentInfoError{source: None, msg: Some("Invalid card".to_owned())})?;
    let number = numbers.pop().ok_or(ParsePaymentInfoError{source: None, msg: Some("Invalid card".to_owned())})?;

    Ok(Card {
        number,
        expiration: Expiration { month, year },
        cvv
    })
}

fn get_credit_card_info(cards: HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    // to convert option to result
    let card_string = cards.get(name).ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}.")))?;

    let card = parse_card(card_string).with_context(|| {
        format!("{name}'s card could not be parsed.")
    }).map_err(|e| {
        CreditCardError::Other(e)
    })?;

    Ok(card)
}

fn main() {
    // init is associated function
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    let card_res = get_credit_card_info(credit_cards, name.trim());

    match card_res {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(e) => println!("{e}"),
                CreditCardError::Other(_) => println!("\n Something went wrong. Try again.")
            }

            log::error!("\n {err:?}");
        }
    }
}
