use hyprland::{data::Monitors, shared::HyprData};

use crate::{help::errors::monitor_not_found_error, monitors::{active_window::is_active_window, workspaces::is_workspaces}};
use super::Options;

pub fn validate_monitor(x: &str, mon: &String) -> bool {
  let flag = x.split_whitespace().next().unwrap_or("");
  (flag.eq("-m") || flag.eq("--monitor")) && (
    is_active_window(mon)
    || is_workspaces(mon)
  )
}

pub fn set_monitor(options: &mut Options, param: String) {
  let monitors = Monitors::get().expect("Failed to retreive monitors data, I'm sorry :(");
  
  for m in monitors {
    if param.eq(&m.id.to_string()) || param.to_uppercase().eq(&m.name.to_uppercase()) {
      options.monitor = Some(m.id);
    }
  }

  if options.monitor.is_none() { monitor_not_found_error(param); }
}

pub fn monitor_dependency_validation(options: &Options, monitor: &String) -> bool {
  is_active_window(monitor) && options.monitor.is_some() && !options.full
}