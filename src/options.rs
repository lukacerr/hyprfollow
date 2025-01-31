use crate::{help::print_monitor_help, monitor::active_window::is_active_window};

#[derive(Debug)]
pub struct Options {
  pub full: bool,
  pub monitor: Option<i128>
}

fn validate_full(x: &str, mon: &String) -> bool {
  (x.eq("-f") || x.eq("--full")) && (
    is_active_window(mon)
  )
}

fn validate_monitor(x: &str, mon: &String) -> bool {
  let flag = x.split_whitespace().next().unwrap_or("");
  (flag.eq("-m") || flag.eq("--monitor")) && (
    is_active_window(mon)
  )
}

fn get_param(args: &Vec<String>, i: &usize, monitor: &String) -> String {
  match args.get(i+1) {
    Some(param) => param.to_owned(),
    // FIXME: Any better way to do this?
    _ => { print_monitor_help(monitor); "".to_string() }
  }
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
      x if validate_full(x, monitor) => options.full = true,

      x if validate_monitor(x, monitor) => {
        let param = get_param(&args, &i, &monitor);

        // TODO: Validate if monitor exists
        // If it does, save id
        // If not, print message including available monitors and exit
        options.monitor = Some(param.parse().expect("Monitor ID required"));
      } 

      _ => print_monitor_help(monitor), 
    };
  }

  // Dependencies validation
  if options.monitor.is_some() && !options.full
    { print_monitor_help(monitor) }


  drop(args);
  options
}