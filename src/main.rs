use serde::{Deserialize, Serialize};
use std::process::Command;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub id: i32,
    pub pid: i32,
    pub app: String,
    pub title: String,
    pub scratchpad: String,
    pub frame: Frame,
    pub role: String,
    #[serde(rename = "subrole")]
    pub sub_role: String,
    #[serde(rename = "root-window")]
    pub root_window: bool,
    pub display: i32,
    pub space: i32,
    pub level: i32,
    #[serde(rename = "sub-level")]
    pub sub_level: i32,
    pub layer: String,
    #[serde(rename = "sub-layer")]
    pub sub_layer: String,
    pub opacity: f32,
    #[serde(rename = "split-type")]
    pub split_type: String,
    #[serde(rename = "split-child")]
    pub split_child: String,
    #[serde(rename = "stack-index")]
    pub stack_index: i32,
    #[serde(rename = "can-move")]
    pub can_move: bool,
    #[serde(rename = "can-resize")]
    pub can_resize: bool,
    #[serde(rename = "has-focus")]
    pub has_focus: bool,
    #[serde(rename = "has-shadow")]
    pub has_shadow: bool,
    #[serde(rename = "has-parent-zoom")]
    pub has_parent_zoom: bool,
    #[serde(rename = "has-fullscreen-zoom")]
    pub has_fullscreen_zoom: bool,
    #[serde(rename = "has-ax-reference")]
    pub has_ax_reference: bool,
    #[serde(rename = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
    #[serde(rename = "is-visible")]
    pub is_visible: bool,
    #[serde(rename = "is-minimized")]
    pub is_minimized: bool,
    #[serde(rename = "is-hidden")]
    pub is_hidden: bool,
    #[serde(rename = "is-floating")]
    pub is_floating: bool,
    #[serde(rename = "is-sticky")]
    pub is_sticky: bool,
    #[serde(rename = "is-grabbed")]
    pub is_grabbed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    pub id: i32,
    pub uuid: String,
    pub index: i32,
    pub label: String,
    #[serde(rename = "type")]
    pub space_type: String,
    pub display: i32,
    pub windows: Vec<i32>,
    #[serde(rename = "first-window")]
    pub first_window: i32,
    #[serde(rename = "last-window")]
    pub last_window: i32,
    #[serde(rename = "has-focus")]
    pub has_focus: bool,
    #[serde(rename = "is-visible")]
    pub is_visible: bool,
    #[serde(rename = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
}

fn get_space_info() -> (i32, i32) {
    let space_output = Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--spaces")
        .output()
        .expect("failed to execute yabai for spaces");

    let space_json_data = String::from_utf8_lossy(&space_output.stdout);
    match serde_json::from_str::<Vec<Space>>(&space_json_data) {
        Ok(spaces) => {
            for space in &spaces {
                if space.has_focus {
                    return (spaces.len() as i32, space.index);
                }
            }
        },
        Err(_) => {}
    }
    (0, 0)
}

fn get_windows(space: i32) -> Option<Vec<Window>> {
    let windows_output;
    if space == -1 {
        windows_output = Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .output()
        .expect("failed to execute yabai");
    } else {
        windows_output = Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .arg("--space")
        .arg(space.to_string())
        .output()
        .expect("failed to execute yabai");
    }

    if !windows_output.status.success() {
        return None
    }

    let json_data = String::from_utf8_lossy(&windows_output.stdout);
    match serde_json::from_str::<Vec<Window>>(&json_data) {
        Ok(windows) => {
            return Some(windows)
        },
        Err(_) => {
            return None
        }
    }
}

fn main() {
    let (max_space, _focused_space) = get_space_info();

    let mut space = -1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<i32>() {
            Ok(n) => {
                if 1 <= n && n <= max_space {
                    space = n;
                }
            },
            Err(_) => (),
        }
    }

    if let Some(windows) = get_windows(space) {
        for window in windows {
            println!("{} {} {:5} \"{}\" \"{}\"", window.id, window.space, window.has_focus, window.app, window.title);
        }
    }
}
