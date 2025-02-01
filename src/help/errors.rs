use super::print_custom_help;

pub fn monitor_not_found_error(param: String) {
  print_custom_help(&format!("\
Couldn't find monitor '{}' :(
Please use 'hyprctl monitors' to check your data
--monitor flag accepts name or ID :D
  ", param));
}