mod util;
mod aoc1;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author, version, about, long_about = None,
    arg_required_else_help(true)
)]
struct Args {
    /// The number of the problem to execute.
    #[arg(
        index=1,
        value_parser=clap::value_parser!(u8).range(1..31))]
    problem: u8,
    /// Prints the problem text instead of executing the solution.
    #[arg(
        short='p',
        long="print",
        default_value_t=false)]
    print: bool,
}

pub fn main() {
    let args = Args::parse();
    if args.print {
        crate::util::problem_print(&format!("{}", args.problem));
    } else {
        match args.problem {
            1 => aoc1::main(),
            _ => println!("I haven't finished that problem yet!"),
        };
    }
}
