use clap::Parser;

const HELP_TEMPLATE: &str = "\
{before-help}{bin} {version}
{author}
{about}

{usage-heading}
{tab}{usage}

{all-args}{after-help}";

#[derive(Parser, Default, Debug)]
#[clap(author, version, about, help_template = HELP_TEMPLATE)]
struct Arguments {
    arg1: String,
    arg2: String,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
