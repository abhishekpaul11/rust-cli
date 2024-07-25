use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct Echo {
    /// Message to be displayed
    message: String,

    /// Case of the displayed message
    #[arg(short = 'c', long = "case", value_name = "CASE")]
    case: Option<Case>,
}

#[derive(ValueEnum, Clone)]
enum Case {
    Upper,
    Lower,
    Title,
    Toggle
}

pub fn parse(echo: Echo) {
    if let Some(case) = echo.case {
        match case {
            Case::Upper => { println!("{}", echo.message.to_uppercase()) }
            Case::Lower => { println!("{}", echo.message.to_lowercase()) }
            Case::Title => { println!("{}", title_case(echo.message)) }
            Case::Toggle => { println!("{}", toggle_case(echo.message)) }
        }
    }
}

fn title_case(message: String) -> String {
    let words = message.split(' ');
    let mut result = String::new();
    for word in words {
        let mut chars = word.chars();
        result += match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str() + " ",
            None => String::new()
        }.as_str()
    }
    result.trim().to_string()
}

fn toggle_case(message: String) -> String {
    let words = message.split(' ');
    let mut result = String::new();
    for word in words {
        let mut chars = word.chars();
        result += match chars.next() {
            Some(c) => c.to_lowercase().collect::<String>() + chars.as_str().to_uppercase().as_str() + " ",
            None => String::new()
        }.as_str()
    }
    result.trim().to_string()
}