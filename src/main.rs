#![cfg_attr(feature = "no_console", windows_subsystem = "windows")]
#![allow(unused_labels)]

use anyhow::Context;
use serde_json::value::Index;
use serde_json::Value;
use std::net::TcpStream;
use tray_item::{IconSource, TrayItem};
use tungstenite::http::Uri;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tray: TrayItem = TrayItem::new(
        "GAT - GlazeWM Alternating Tiler",
        IconSource::Resource("main-icon"),
    )?;
    tray.add_label("GAT - GlazeWM Alternating Tiler")?;
    tray.add_menu_item("Quit GAT", || std::process::exit(0))?;

    let (mut socket, _) = connect(
        "ws://localhost:6123"
            .parse::<Uri>()
            .context("Failed to parse GWM WS URL")?,
    )
    .context("Failed to connect to GWM WS")?;

    socket
        .send(Message::Text(r#"sub -e focus_changed"#.into()))
        .context("Failed to subscribe to focus_changed event")?;

    loop {
        let (x, y) = {
            let json_msg = match read_as_json(&mut socket) {
                Some(value) => value,
                None => continue,
            };

            let x = json_msg
                .get_path(["data", "focusedContainer", "width"])
                .and_then(|v| v.as_f64());
            let y = json_msg
                .get_path(["data", "focusedContainer", "height"])
                .and_then(|v| v.as_f64());

            match (x, y) {
                (Some(x), Some(y)) => (x, y),
                _ => continue, // skip messages that don't contain the required data
            }
        };

        if x < y {
            let _ = socket.send(Message::Text(
                "command set-tiling-direction vertical".into(),
            ));
        }
        if x > y {
            let _ = socket.send(Message::Text(
                "command set-tiling-direction horizontal".into(),
            ));
        }
    }
}

fn read_as_json(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Option<Value> {
    let msg = match socket.read() {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Error while reading message: {err}");
            return None;
        }
    };

    let text = match msg.to_text() {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error while converting message to text: {err}");
            return None;
        }
    };

    let json_msg = match text.parse::<Value>() {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Error while parsing message as json: {err}");
            return None;
        }
    };

    Some(json_msg)
}

trait JsonValueExt {
    /// Retrieves a nested value based on the provided path of keys.
    ///
    /// # Arguments
    /// * `path` - An iterable of string keys specifying the nested path.
    ///
    /// # Returns
    /// * `Option<&Value>` - The nested value if found, otherwise `None`.
    fn get_path<T: IntoIterator<Item = I>, I: Index>(&self, path: T) -> Option<&Value>;
}

impl JsonValueExt for Value {
    fn get_path<T: IntoIterator<Item = I>, I: Index>(&self, path: T) -> Option<&Value> {
        path.into_iter()
            .fold(Some(self), |acc, key| acc.and_then(|v| v.get(key)))
    }
}
