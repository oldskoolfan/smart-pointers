use std::env;

use smart_pointers::{run_deref, run_drop, run_cons, run_leak, run_tree};

fn main() {
  if let Some(program_name) = get_program_arg() {
    run_program(&program_name);
  } else {
    print_help();
  }
}

fn run_program(name: &str) {
  println!("Running program {}", name);

  match name {
    "deref" => run_deref(),
    "drop" => run_drop(),
    "cons" => run_cons(),
    "leak" => run_leak(false),
    "leakkk" => run_leak(true),
    "tree" => run_tree(),
    &_ => print_help(),
  }
}

fn print_help() {
  println!("Please specify a program to run -- deref, drop, cons, leak");
}

fn get_program_arg() -> Option<String> {
  let mut args = env::args();
  let _ = args.next(); // always target/debug/smart_pointers

  args.next() // program name arg
}
