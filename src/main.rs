use clap::{Parser, Subcommand};
use std::process::Command;
use yansi::Paint;

const HELP_TEMPLATE: &str = "\
{before-help}{bin} {version}
by {author}
{about}

{usage-heading}
{tab}{usage}

{all-args}{after-help}";

// let output = zsh.output().expect("Failed to execute command");
// println!("{:?}", output.stdout.as_slice());

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, help_template = HELP_TEMPLATE)]
struct Arguments {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Pull,
    Push,
}

// macro_rules! run_command {
//     ($cmd:expr) => {
//         let format_failed_message = "failed to covert command to string";
//         let mut zsh = Command::new("zsh");
//         zsh.args(["-i", "-c", $cmd]);
//         zsh.current_dir(home::home_dir().expect("failed to get home directory"));
//         let status = zsh.status().expect(
//             format_args!("{} failed to execute", $cmd)
//                 .as_str()
//                 .expect(format_failed_message),
//         );
//         match status.code().expect(
//             format_args!("no status code from {}", $cmd)
//                 .as_str()
//                 .expect(format_failed_message),
//         ) {
//             0 => println!("=> ✓ {}", $cmd),
//             _ => {
//                 eprintln!("=> ✗ {} failed", $cmd);
//                 panic!("Command failed");
//             }
//         }
//     };
// }

fn run_command(cmd: &str) {
    println!("→ {}", cmd.bold());
    let mut zsh = Command::new("zsh");
    zsh.env("SHELL_SESSIONS_DISABLE", "1");
    zsh.args(["-i", "-c", cmd]);
    zsh.current_dir(home::home_dir().expect("failed to get home directory"));
    let status = zsh
        .status()
        .expect(format!("command {} failed to execute", cmd).as_str());
    let status_code = status
        .code()
        .expect(format!("no status code from command {}", cmd).as_str());
    match status_code {
        0 => println!("{} {}", "✓ success".green(), format!("for `{}`", cmd).dim()),
        _ => {
            eprintln!(
                "{} {}",
                "✗ failed".red(),
                format!(
                    "when running `{}`: command returned code {}",
                    cmd, status_code
                )
                .dim(),
            );
            panic!("Command exited with nonzero code");
        }
    }
}

fn pull() {
    println!("{}", "Hi, welcome back!".dim());
    run_command("chezmoi apply ~/Brewfile --force");
    run_command("brew bundle install");
    run_command("return 1");
    println!("{}", "You're all set.".dim());
}

fn push() {
    println!("{}", "Closing up shop…".dim());
    run_command("brew bundle dump --force");
    run_command("chezmoi add ~/Brewfile");
    run_command("chezmoi git add .");
    run_command("chezmoi git commit -- -m 'Automated commit from salut'");
    run_command("chezmoi git push");
    println!("{}", "Goodbye!".dim());
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        Commands::Pull => pull(),
        Commands::Push => push(),
    }
}
