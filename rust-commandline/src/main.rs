use clap::{Args, Parser, Subcommand};

// can't rename `Option`
#[derive(Parser)]
#[command(version = "1.0", author = "snowan")]
struct Opts {
    // hello!
    #[arg(short, long, default_value = "hello world!")]
    greet_message: String,
    // anything
    anything: String,
    // subcommand
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(version = "1.3", author = "snowman")]
    Run(Mode),
}

#[derive(Args)]
struct Mode {
    // if true, dry run
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("anything: {}", opts.anything);
    println!("hello! {}", opts.greet_message);

    match opts.subcmd {
        SubCommand::Run(t) => {
            if t.dry_run {
                println!("rollback");
            } else {
                println!("commit");
            }
        }
    }
}
