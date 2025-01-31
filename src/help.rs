use crate::monitor::active_window::{is_active_window, ACTIVE_WINDOW_HELP};

const GENERIC_HELP: &str = "\
HyprFollow: Monitor your Hyprland activity.

USAGE:
  hyprfollow [monitor] [options...]

FLAGS:
  -h, --help                    Displays this text

AVAILABLE MONITORS:
  aw, active-window             Monitors active window
  w, workspaces                 Monitors workspaces changes

FLAGS FOR ALL MONITORS:
  -h, --help                    Displays list of options for the specified monitor

EXAMPLES:
  hyprfollow aw                 Starts monitoring active window state
  hyprfollow w -h               Lists options for the workspaces monitor

TIPS:
  Combine with jq for a readable output: hyprfollow [monitor] [options...] | jq .

> May I help you?
@lukacerr in most social media :)
";

fn print_help_and_exit(help_str: &str){
  println!("{}", help_str);
  std::process::exit(0);
}

pub fn arg_is_help(arg: &String) -> bool {
  arg.eq("h") || arg.eq("-h") || arg.eq("--help") || arg.eq("help") || arg.eq("-")
}

pub fn print_generic_help_and_exit(){
  print_help_and_exit(GENERIC_HELP);
}

pub fn print_monitor_help(monitor: &String) {
  print_help_and_exit(match monitor {
    x if is_active_window(x) => ACTIVE_WINDOW_HELP,
    _ => GENERIC_HELP
  });
}