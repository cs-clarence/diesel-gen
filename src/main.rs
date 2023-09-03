use clap::Parser;

mod cli;
mod config;
mod generate;
mod parse;

fn print_diesel_dependencies() {
  println!("use diesel::{{Queryable, Insertable, Selectable, Identifiable, AsChangeset}};");
}

fn print_rust_file_headers() {
  println!("// @generated \n");
  println!("// Generated by diesel-gen\n");

  println!("#![allow(unused)]");
  println!("#![allow(clippy::all)]\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = cli::Cli::parse();

  match args.command {
    cli::CliSubcommand::Model => {
      print_rust_file_headers();
      print_diesel_dependencies();
    }
  }

  Ok(())
}
