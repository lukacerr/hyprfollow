use hyprland::{async_closure, event_listener::AsyncEventListener};
use serde::Serialize;
use serde_json::json;

use crate::options::Options;

pub fn is_submap(monitor: &String) -> bool {
  monitor.eq("s") || monitor.eq("submap") || monitor.starts_with("sm")
}

pub const SUBMAP_HELP: &str = "\
HyprFollow - Submap monitor.
Listens when submap changes.

USAGE:
  hyprfollow (s|submap) [options...]

FLAGS:
  -h, --help                    Displays this text

EXAMPLES:
  hyprfollow s                  Starts monitoring submap

TIPS:
  Combine with jq for a readable output: hyprfollow [monitor] [options...] | jq .
  Concatenate more than one monitor with the '&' operator (ej 'hyprfollow s & hyprfollow w -m 3')
";

#[derive(Serialize)]
struct Submap {
  pub title: Option<String>,
}

async fn print_submap(value: String) {
  println!("{}", 
    json!(Submap { title: match value.is_empty() {
      false => Some(value),
      _ => None
    }})
  )
}

pub async fn submap_monitor(_: Options) -> hyprland::Result<()> {
  let mut event_listener = AsyncEventListener::new();

  print_submap("".to_string()).await;

  event_listener.add_sub_map_changed_handler(
    async_closure! { |value| print_submap(value).await }
  );
  
  event_listener.start_listener_async().await?;
  Ok(())
}