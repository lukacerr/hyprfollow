use hyprland::{async_closure, data::{Client, Monitor}, event_listener::{AsyncEventListener, WindowEventData}, shared::{HyprDataActive, HyprDataActiveOptional}};
use serde::Serialize;
use serde_json::json;

use crate::options::Options;

pub fn is_active_window(monitor: &String) -> bool {
  monitor.eq("aw") || monitor.eq("active-window")
}

pub const ACTIVE_WINDOW_HELP: &str = "\
HyprFollow - Active window monitor.
Uses 'active_window_changed_handler' from the Rust bindings.

USAGE:
  hyprfollow (aw|active-window) [options...]

FLAGS:
  -h, --help                    Displays this text
  -f, --full                    Expands info at the expense of re-fetching (read below)
  -m, --monitor <MONITOR>       Filter by monitor's ID or name (REQUIRES --full TO WORK)

EXAMPLES:
  hyprfollow active-window      Starts monitoring active window state
  hyprfollow aw -f -m DP-1    Monitors active window on specific monitor, retreiving full data

TIPS:
  Combine with jq for a readable output: hyprfollow aw [options...] | jq .

> WHY ISN'T '--full' THE DEFAULT?
Some of the data that provides require re-fetching.
This is due to the fact that the event data doesn't provide all the information related.
The performance impact is not really significant, but I want people to be aware of it.
";

#[derive(Serialize)]
struct ActiveWindow {
  pub class: String,
  pub title: String,
}

enum ToActiveWindow {
  Client(Client),
  Window(WindowEventData),
  None()
}

async fn build_active_window(value: ToActiveWindow) -> ActiveWindow {
  match value {
    ToActiveWindow::Client(x) => ActiveWindow { 
      title: x.title,
      class: x.class,
    },
    ToActiveWindow::Window(x) => ActiveWindow { 
      title: x.title,
      class: x.class,
    },
    ToActiveWindow::None() => ActiveWindow { 
      title: "-".to_string(),
      class: "".to_string(),
    },
  }
}

async fn print_active_window(window: ActiveWindow) {
  println!("{}", json!(window))
}

async fn print_active_window_full(window: Client, monitor: Monitor) {
  let mut window_json = json!(window);
  window_json["monitorName"] = json!(monitor.name);
  println!("{}", window_json)
}

async fn get_client() -> (Option<Client>, Monitor) {
  (Client::get_active_async().await
    .expect("Failed to access active window data :("), 
  Monitor::get_active_async().await
  .expect("Failed to access active monitor data :("))
}

pub async fn active_window_monitor(options: Options) -> hyprland::Result<()> {
  let (initial_client, initial_monitor) = get_client().await;
  let mut event_listener = AsyncEventListener::new();

  if options.full { 
    if initial_client.is_some() && options.monitor.is_none_or(|id| id.eq(&initial_monitor.id)) {
      print_active_window_full(initial_client.unwrap(), initial_monitor).await; 
    } else { print_active_window(build_active_window(ToActiveWindow::None()).await).await; }

    event_listener.add_active_window_changed_handler(async_closure! { move |_| 
      match get_client().await {
        (Some(client), monitor) => {
          if options.monitor.is_none() || options.monitor == client.monitor { 
            print_active_window_full(client, monitor).await 
          }
          else {()} 
        },
        _ => () 
      }
    });
  }

  else { 
    if initial_client.is_some() {
      print_active_window(build_active_window(ToActiveWindow::Client(initial_client.unwrap())).await).await; 
    }

    event_listener.add_active_window_changed_handler(async_closure! { 
      |event| 
        print_active_window(
          build_active_window(
            match event {
              Some(window) => ToActiveWindow::Window(window),
              None => ToActiveWindow::None(),
            }
          ).await
        ).await
    });
  }

  event_listener.start_listener_async().await?;
  Ok(())
}
