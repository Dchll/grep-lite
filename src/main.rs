use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(
    version = "0.0.1",
    about = "searches for patterns",
    long_about = "by 多吃韩66"
)]
struct Args {
    #[arg(help = "The pattern to search for")]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;

    let re = Regex::new(pattern.as_str()).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and 
dark square is a picture feverishly turned--in search of what?
it is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            None => (),
            Some(_) => println!("{}", line),
        }
    }
}
