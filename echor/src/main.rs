use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None, about="Rust echo")]
struct Args {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,
    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
    /// Separator
    #[arg(short, long, default_value = " ")]
    sep: String,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(&args.sep),
        if args.omit_newline { "" } else { "\n" }
    );
}
