use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use druid::AppDelegate;
use druid::DelegateCtx;
use druid::ExtEventSink;
use druid::Handled;
use druid::Selector;
use druid::Target;
use druid::{commands, Command, Env};

use sha2::Sha256;

use crate::data::{AppState, WhichFileAreWeOpening};
use crate::hash;

// pub const START_SLOW_FUNCTION: Selector = Selector::new("start_slow_function");

pub const FINISH_SLOW_FUNCTION: Selector<String> = Selector::new("finish_slow_function");

pub fn wrapped_slow_function(sink: ExtEventSink, path: Arc<PathBuf>) {
    thread::spawn(move || {
        let hash_result = hash::hasher::<Sha256>(&path).expect("hashing failed oh no!");
        sink.submit_command(FINISH_SLOW_FUNCTION, hash_result, Target::Auto)
            .expect("command failed to submit");
    });
}

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(hash_result) = cmd.get(FINISH_SLOW_FUNCTION) {
            data.processing = false;
            data.hash_result = hash_result.clone().into();
            Handled::Yes
        } else if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match data.which_file_kind {
                WhichFileAreWeOpening::TheBinary => {
                    data.path = Some(Arc::new(file_info.path().into()));
                    if let Some(name) = file_info.path().file_name() {
                        data.filename = name.to_string_lossy().into();
                    }
                }
                WhichFileAreWeOpening::TheManifest => {
                    data.manifest_path = Some(Arc::new(file_info.path().into()));
                    if let Some(name) = file_info.path().file_name() {
                        data.manifest_filename = name.to_string_lossy().into();
                    }
                }
            }

            druid::Handled::Yes
        } else {
            Handled::No
        }
    }
}
