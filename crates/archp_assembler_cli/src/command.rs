use clap::{
    Parser,
    ValueHint::FilePath,
    builder::{Styles, styling::AnsiColor},
};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
#[command(styles = get_styles())]
pub struct Cli {
    /// File path to the source assembly file.
    #[arg(value_hint = FilePath)]
    pub src_file: String,

    /// The output file path.
    #[arg(short, long, value_hint = FilePath, default_value = "a.o")]
    pub out_file: String,

    /// Output to stdout
    #[arg(long)]
    pub stdout: bool,

    /// Output formatted hex instead of binary machine code.
    #[arg(long)]
    pub hex: bool,

    /// Disable the macro-instructions.
    #[arg(long)]
    pub disable_macro: bool,
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
        .context(AnsiColor::Cyan.on_default())
        .context_value(AnsiColor::Magenta.on_default())
}
