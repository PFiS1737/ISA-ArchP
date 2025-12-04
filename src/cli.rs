use clap::{
    Parser,
    ValueHint::{DirPath, FilePath},
    builder::{Styles, styling::AnsiColor},
};
use clap_complete::Shell;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
#[command(styles = get_styles())]
pub struct Cli {
    /// Print shell auto completions for the specified shell.
    #[arg(long)]
    pub complete: Option<Shell>,

    #[arg(value_hint = FilePath)]
    pub src_file: String,

    #[arg(value_hint = DirPath)]
    pub dist_dir: Option<String>,
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold().underline())
        .usage(AnsiColor::Yellow.on_default().bold().underline())
        .valid(AnsiColor::Green.on_default().bold().underline())
        .invalid(AnsiColor::Red.on_default().bold())
        .placeholder(AnsiColor::White.on_default())
        .error(AnsiColor::Red.on_default().bold())
        .literal(AnsiColor::Green.on_default())
}
