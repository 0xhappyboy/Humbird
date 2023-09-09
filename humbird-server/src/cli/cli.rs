use clap::Parser;

#[derive(Parser)]
#[command(name = "Humbird", author = "HappyBoy", version = "0.1.0",about="you know,for faster", long_about=None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(long, short, help = "server port (default: 9999).")]
    port: Option<String>,
}
