use druid::ArcStr;
use druid::EventCtx;
use std::collections::HashMap;
use std::sync::Arc;
use std::{fs, path::PathBuf};

use crate::{delegate, manifest_parser};
use druid::{commands, Data, Env, FileDialogOptions, Lens};

#[derive(Clone, Data, PartialEq)]
pub enum WhichFileAreWeOpening {
    TheBinary,
    TheManifest,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub processing: bool,
    pub hash_result: ArcStr,
    pub filename: ArcStr,
    pub manifest_filename: ArcStr,
    pub path: Option<Arc<PathBuf>>,
    pub manifest_path: Option<Arc<PathBuf>>,
    pub expected_hash: String,
    pub match_result: ArcStr,
    pub which_file_kind: WhichFileAreWeOpening,
    pub filename_hash_pairs: Arc<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            processing: false,
            hash_result: "".into(),
            filename: "".into(),
            manifest_filename: "".into(),
            path: None,
            manifest_path: None,
            expected_hash: "".into(),
            match_result: "".into(),
            which_file_kind: WhichFileAreWeOpening::TheBinary,
            filename_hash_pairs: Arc::new(HashMap::new()),
        }
    }

    pub fn choose_file(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.which_file_kind = WhichFileAreWeOpening::TheBinary;
        ctx.submit_command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()));
    }

    pub fn choose_manifest(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.which_file_kind = WhichFileAreWeOpening::TheManifest;
        ctx.submit_command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()));
    }

    pub fn parse_manifest(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if let Some(manifest_path) = data.manifest_path.clone() {
            let path: &PathBuf = &manifest_path.clone();
            let manifest = fs::read_to_string(path).unwrap();
            if let Ok(map) = manifest_parser::parse_manifest(manifest, true) {
                data.filename_hash_pairs = Arc::new(map)
            } else {
                println!("failed to parse manifest");
            };
        } else {
            println!("NO MANIFEST YET DUMMY");
        }
    }

    pub fn pick_expected_hash(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.filename.is_empty() {
            println!("no file selected")
        } else if data.manifest_path.is_none() {
            println!("no manifest selected")
        } else if data.filename_hash_pairs.is_empty() {
            println!("didn't parse the manifest yet maybe")
        } else {
            if let Some(hash) = data.filename_hash_pairs.get(data.filename.as_ref().clone()) {
                data.expected_hash = hash.clone()
            } else {
                println!("no matching hash for {} found", data.filename);
                for (name, hash) in &data.filename_hash_pairs.as_ref().clone() {
                    println!("name {} doesn't match {}", name, data.filename);
                }
            }
        }
    }

    pub fn start_hash(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.processing = true;
        if let Some(path) = data.path.clone() {
            delegate::wrapped_slow_function(ctx.get_external_handle(), path);
        }
    }

    pub fn compare_hash(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.hash_result.to_string().trim() == data.expected_hash.trim() {
            data.match_result = "YES".into();
        } else {
            data.match_result = "NO".into();
        }
    }
}
