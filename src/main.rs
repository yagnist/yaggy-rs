
mod cli;

fn main() {
    let cli_args = cli::cli();

    println!("{:?}", cli_args);
}
