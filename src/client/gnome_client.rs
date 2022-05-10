use std::collections::HashMap;

use zbus::{blocking::Connection, zvariant::Value};

use crate::client::Client;
use serde::{Deserialize, Serialize};

pub struct GnomeClient {
    connection: Option<Connection>,
}

impl GnomeClient {
    pub fn new() -> GnomeClient {
        GnomeClient { connection: None }
    }

    fn connect(&mut self) {
        match Connection::session() {
            Ok(connection) => self.connection = Some(connection),
            Err(e) => println!("GnomeClient#connect() failed: {}", e),
        }
    }
}

impl Client for GnomeClient {
    fn supported(&mut self) -> bool {
        self.connect();
        self.current_application().is_some()
    }

    fn current_application(&mut self) -> Option<String> {
        self.connect();
        let connection = match &mut self.connection {
            Some(connection) => connection,
            None => return None,
        };

        let safe_introspected = connection
            .call_method(
                Some("org.gnome.Shell"),
                "/dev/wxwee/SafeIntrospect",
                Some("dev.wxwee.SafeIntrospect"),
                "GetWindows",
                &(),
            )
            .map_err(|e| {
                eprintln!("Calling SafeIntrospection failed. Attempting to call the rest of the strategies.");
                e
            })
            .ok()
            .and_then(|message| {
                let windows = message
                    .body::<HashMap<u64, HashMap<String, Value<'_>>>>()
                    .map_err(|err| {
                        eprintln!("Error deserializing body: {:?}. Message: {message:?}", err);
                        err
                    })
                    .ok()?;

                let focused_window = windows.iter().find(|(_window_id, properties)| {
                    properties
                        .get("has-focus")
                        .map(|val| {
                            if let &Value::Bool(bool_val) = val {
                                bool_val
                            } else {
                                eprintln!(
                                    "Unexpectedly did not get boolean value from has-focus. Got {val:?} instead."
                                );
                                false
                            }
                        })
                        .unwrap_or(false)
                });

                let wm_class = focused_window
                    .and_then(|(_window_id, properties)| properties.get("wm-class"))
                    .and_then(|wm_class| {
                        if let Value::Str(wm_class_str) = wm_class {
                            Some(wm_class_str.to_string())
                        } else {
                            eprintln!("Unexpectedly did not get string value from wm-class. Got {wm_class:?} instead.");
                            None
                        }
                    });

                wm_class
            });

        if safe_introspected.is_some() {
            return safe_introspected;
        }

        // Default strategies below:

        // Attempt the latest protocol
        if let Ok(message) = connection.call_method(
            Some("org.gnome.Shell"),
            "/com/k0kubun/Xremap",
            Some("com.k0kubun.Xremap"),
            "ActiveWindow",
            &(),
        ) {
            if let Ok(json) = message.body::<String>() {
                if let Ok(window) = serde_json::from_str::<ActiveWindow>(&json) {
                    return Some(window.wm_class);
                }
            }
        // Fallback to the legacy protocol
        } else if let Ok(message) = connection.call_method(
            Some("org.gnome.Shell"),
            "/com/k0kubun/Xremap",
            Some("com.k0kubun.Xremap"),
            "WMClass",
            &(),
        ) {
            if let Ok(wm_class) = message.body::<String>() {
                return Some(wm_class);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
struct ActiveWindow {
    #[serde(default)]
    wm_class: String,
    #[serde(default)]
    title: String,
}
