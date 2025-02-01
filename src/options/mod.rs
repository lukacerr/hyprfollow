use full_option::{set_full, validate_full};
use monitor_option::{monitor_dependency_validation, set_monitor, validate_monitor};

use crate::help::print_monitor_help;

pub mod full_option;
pub mod monitor_option;

#[derive(Debug)]
pub struct Options {
  pub full: bool,
  pub monitor: Option<i128>
}

fn get_param(args: &Vec<String>, i: &usize, monitor: &String) -> String {
  let param = args.get(i+1);
  if param.is_none() { print_monitor_help(monitor); }
  param.unwrap().to_owned()
}

pub fn args_to_options(args: Vec<String>, monitor: &String) -> Options {
  let mut options = Options {
    full: false,
    monitor: None
  };

  for i in (0 .. args.len()).rev() { 
    let arg = &args[i];
    if !arg.starts_with("-") { continue; }
  
    match arg {
      x if validate_full(x, monitor) => set_full(&mut options),
      x if validate_monitor(x, monitor) => set_monitor(&mut options, get_param(&args, &i, monitor)),
      // x if validate_new_param(x, monitor) => { options.new_param = ? } 
      _ => print_monitor_help(monitor), 
    };
  }

  if monitor_dependency_validation(&options, &monitor)
    { print_monitor_help(monitor) }

  options
}