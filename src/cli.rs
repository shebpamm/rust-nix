use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "{{ project-name }}", version)]
pub struct Arguments {}

pub fn parse_args() -> Arguments {
    Arguments::parse()
}
