use std::io;
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    SquareRoot,
}

impl Operation {
    fn apply(&self, first: &BigDecimal, second: &BigDecimal) -> Result<BigDecimal, String> {
        match self {
            Operation::Add => Ok(first + second),
            Operation::Subtract => Ok(first - second),
            Operation::Multiply => Ok(first * second),
            Operation::Divide => {
                if second.is_zero() {
                    Err("Error: Division by zero".to_string())
                } else {
                    Ok(first / second)
                }
            },
            Operation::Modulus => {
                if second.is_zero() {
                    Err("Error: Division by zero in modulus".to_string())
                } else {
                    Ok(first % second)
                }
            },
            Operation::Power => Ok(self.power(first, second)),
            Operation::SquareRoot => {
                if first < &BigDecimal::from(0) {
                    Err("Error: Cannot compute the square root of a negative number".to_string())
                } else {
                    match first.sqrt() {
                        Some(result) => Ok(result),
                        None => Err("Error: Unable to compute square root".to_string()),
                    }
                }
            },
        }
    }

    fn power(&self, base: &BigDecimal, exponent: &BigDecimal) -> BigDecimal {
        let mut result = BigDecimal::from(1);
        let exp = exponent.to_u32().unwrap_or(0); // Convert exponent to u32
        for _ in 0..exp {
            result = result * base;
        }
        result
    }
}

fn get_number_input() -> BigDecimal {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if let Ok(number) = BigDecimal::from_str(input.trim()) {
            return number;
        }
        println!("Invalid input. Please enter a valid number (either integer or floating point).");
    }
}

fn get_choice_input() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid integer choice."),
        }
    }
}

fn leaning() -> bool {
    loop {
        println!("Would you like to continue or quit?\n1.) Continue | 2.) Quit");
        let choice = get_choice_input();
        match choice {
            1 => return true,
            2 => return false,
            _ => println!("Invalid input, please enter 1 for Continue or 2 for Quit."),
        }
    }
}

fn format_result(result: &BigDecimal) -> String {
    let result_str = result.to_string();

    if result_str.contains('.') {
        let trimmed_result = result_str.trim_end_matches('0').trim_end_matches('.');
        if trimmed_result.is_empty() {
            return "0".to_string();
        } else {
            return trimmed_result.to_string();
        }
    }
    result_str
}

fn main() {
    let mut lean = true;

    while lean {
        println!("Please choose the desired operation:");
        println!("1.) Add 2.) Subtract 3.) Multiply 4.) Divide 5.) Modulus 6.) Power 7.) Square Root");

        let transaction = get_choice_input();

        let first: BigDecimal;
        let second: Option<BigDecimal>;

        match transaction {
            1 | 2 | 3 | 4 | 5 | 6 => {
                println!("Please enter the first number:");
                first = get_number_input();

                println!("Please enter the second number:");
                second = Some(get_number_input());
            },
            7 => {
                println!("Please enter the number:");
                first = get_number_input();
                second = None;
            },
            _ => {
                println!("Please choose a valid transaction!");
                continue;
            },
        }

        if transaction == 7 {
            match Operation::SquareRoot.apply(&first, &BigDecimal::from(0)) {
                Ok(result) => println!("Square root result is: {}", format_result(&result)),
                Err(e) => println!("{}", e),
            };
        } else if let Some(second_value) = second {
            let operation = match transaction {
                1 => Operation::Add,
                2 => Operation::Subtract,
                3 => Operation::Multiply,
                4 => Operation::Divide,
                5 => Operation::Modulus,
                6 => Operation::Power,
                _ => unreachable!(),
            };

            match operation.apply(&first, &second_value) {
                Ok(result) => println!("Result is: {}", format_result(&result)),
                Err(e) => println!("{}", e),
            };
        }

        lean = leaning();
    }
}
