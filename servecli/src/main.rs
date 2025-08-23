use clap::Parser;
use colog::format::CologStyle;
use color_eyre::eyre::Result;
use env_logger::fmt::Formatter;
use log::Level;
use std::io::Write;

use crate::cli::Cli;

mod cli;
mod cmd_run;

pub(crate) struct CustomLoggingStyle;

impl CologStyle for CustomLoggingStyle {
    fn prefix_token(&self, level: &Level) -> String {
        format!("{}", self.level_color(level, self.level_token(level)),)
    }

    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR ",
            Level::Warn => "WRN ",
            Level::Debug => "DBG ",
            _ => "",
        }
    }

    fn format(
        &self,
        buf: &mut Formatter,
        record: &log::Record<'_>,
    ) -> std::result::Result<(), std::io::Error> {
        let prefix = self.prefix_token(&record.level());
        writeln!(buf, "{}{}", prefix, record.args().to_string(),)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    color_eyre::install()?;

    let args = Cli::parse();

    let log_level_filter = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    colog::default_builder()
        .filter_level(log_level_filter)
        .format(colog::formatter(CustomLoggingStyle))
        .target(env_logger::Target::Stdout)
        .init();

    match args.command {
        cli::Commands::Run {} => cmd_run::run().await?,
    }

    Ok(())
}
