#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};

#[derive(Default)]
struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
}

plugin_main!(BasicPlugin); // Important!