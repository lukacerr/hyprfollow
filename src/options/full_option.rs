use crate::monitors::{active_window::is_active_window, workspaces::is_workspaces};
use super::Options;

pub fn validate_full(x: &str, mon: &String) -> bool {
  (x.eq("-f") || x.eq("--full")) && (
    is_active_window(mon)
    || is_workspaces(mon)
  )
}

pub fn set_full(options: &mut Options) {
  options.full = true
}