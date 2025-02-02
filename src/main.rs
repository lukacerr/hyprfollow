use std::env;

pub mod monitors;
use monitors::{active_window::{active_window_monitor, is_active_window}, submap::{is_submap, submap_monitor}, workspaces::{is_workspaces, workspaces_monitor}};

pub mod options;
use options::args_to_options;

pub mod help;
use help::{arg_is_help, print_generic_help_and_exit, print_monitor_help};

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 { print_generic_help_and_exit(); }

  let monitor = &args[1];
  if monitor.is_empty() || arg_is_help(monitor) { 
    print_generic_help_and_exit();
  }
  
  let option_args = &args[2..];
  if option_args.len() > 0 && arg_is_help(&option_args[0]) {
    print_monitor_help(monitor);
  }

  let options = args_to_options(option_args.to_owned(), monitor);

  match monitor {
    x if is_active_window(x) => active_window_monitor(options).await?,
    x if is_workspaces(x) => workspaces_monitor(options).await?,
    x if is_submap(x) => submap_monitor(options).await?,
    // x if is_new(x) => new_monitor(options).await?,
    _ => print_generic_help_and_exit()
  }

  Ok(())
}
