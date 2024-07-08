use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Type {
    B = 2,
    O = 8,
    X = 16,
    D = 10,
}

#[derive(Parser)]
#[command(
    version,
    about = "boxd - converts between [b]inary, [o]ctal, he[x]adecimal, and [d]ecimal representations."
)]
struct Cli {
    #[arg(short, long, value_enum)]
    from: Type,
    #[arg(short, long, value_enum)]
    to: Type,
    value: String,
}

fn main() {
    let cli = Cli::parse();

    let strippable_prefix = match cli.from {
        Type::B => cli.value.strip_prefix("0b"),
        Type::O => cli.value.strip_prefix("0o"),
        Type::X => cli.value.strip_prefix("0x"),
        Type::D => cli.value.strip_prefix("0d"),
    };

    let sanitized = if strippable_prefix.is_some() { strippable_prefix.unwrap() } else { &cli.value };

    let value: u128 = match u128::from_str_radix(sanitized, cli.from as u32) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    };

    print!("[ Resulting ] ");
    match cli.to {
        Type::B => println!("Binary: {:#b}", value),
        Type::D => println!("Decimal: {}", value),
        Type::O => println!("Octal: {:#o}", value),
        Type::X => println!("Hexadecimal: {:#X}", value),
    }
}
