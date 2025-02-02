use hyprland::{async_closure, data::{Workspace, Workspaces}, event_listener::AsyncEventListener, shared::{HyprData, HyprDataActive, WorkspaceId}};
use serde::Serialize;
use serde_json::json;

use crate::options::Options;

pub fn is_workspaces(monitor: &String) -> bool {
  monitor.eq("w") || monitor.eq("ws") || monitor.starts_with("workspace")
}

pub const WORKSPACES_HELP: &str = "\
HyprFollow - Workspaces monitor.
Listens when workspaces are added, deleted, moved, changed and renamed.

USAGE:
  hyprfollow (w|workspaces) [options...]

FLAGS:
  -h, --help                    Displays this text
  -m, --monitor <MONITOR>       Filter by monitor's ID or name (REQUIRES --full TO WORK)

EXAMPLES:
  hyprfollow w                  Starts monitoring active window state
  hyprfollow w -m DP-1          Monitors workspace activity for monitor DP-1, retrieving full data

TIPS:
  Combine with jq for a readable output: hyprfollow [monitor] [options...] | jq .
  Concatenate more than one monitor with the '&' operator (ej 'hyprfollow w -m DP-1 & hyprfollow w -m 3')
";

/* TODO: Implement virtual workspace dom without refetching
#[derive(Serialize)]
struct VirtualWorkspace {
  pub id: WorkspaceId,
  pub name: String,
  pub active: bool,
  pub is_special: Option<bool> 
}
*/

#[derive(Serialize)]
struct WorkspaceExtended {
  pub data: Workspace,
  pub active: bool,
}

async fn print_workspaces_full(workspaces: Workspaces, active: WorkspaceId, monitor_id: Option<i128>) {
  let workspaces_iter = workspaces.iter()
    .filter(|w| monitor_id.is_none_or(|id| 
      w.monitor_id.is_some() && id.eq(&w.monitor_id.unwrap()))
    );

  let mut workspaces_vec: Vec<WorkspaceExtended> = workspaces_iter.map(
    |w| WorkspaceExtended {
      data: w.to_owned(),
      active: w.id == active
  }).collect();

  drop(workspaces);

  workspaces_vec.sort_by_key(|w| w.data.id);

  println!("{}", json!(workspaces_vec));
}

pub async fn workspaces_monitor(options: Options) -> hyprland::Result<()> {
  let mut event_listener = AsyncEventListener::new();
  
  let fetch_and_print = async_closure! { move || {
    let active_op = Workspace::get_active_async().await;
    if active_op.is_err() { return () }

    let active = active_op.unwrap();
    if options.monitor.is_some() && options.monitor.ne(&active.monitor_id) { return () }

    match Workspaces::get_async().await {
      Ok(workspaces) => print_workspaces_full(workspaces, active.id, options.monitor).await,
      _ => {()}
    } 
  }};

  fetch_and_print().await;
  event_listener.add_workspace_added_handler(move |_| fetch_and_print());
  event_listener.add_workspace_changed_handler(move |_| fetch_and_print());
  event_listener.add_workspace_deleted_handler(move |_| fetch_and_print());
  event_listener.add_workspace_moved_handler(move |_| fetch_and_print());
  event_listener.add_workspace_renamed_handler(move |_| fetch_and_print());
  if options.monitor.is_none() {
    event_listener.add_active_monitor_changed_handler(move |_| fetch_and_print());
  }

  event_listener.start_listener_async().await?;
  Ok(())
}
