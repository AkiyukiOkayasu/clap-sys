use crate::{host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_REMOTE_CONTROLS: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.remote-controls.draft/2\0") };

pub const CLAP_REMOTE_CONTROLS_COUNT: usize = 8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_remote_controls_page {
    pub section_name: [c_char; CLAP_NAME_SIZE],
    pub page_id: clap_id,
    pub page_name: [c_char; CLAP_NAME_SIZE],
    pub param_ids: [clap_id; CLAP_REMOTE_CONTROLS_COUNT],
    pub is_for_preset: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_remote_controls {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            page_index: u32,
            page: *mut clap_remote_controls_page,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_remote_controls {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub suggest_page: Option<unsafe extern "C" fn(host: *const clap_host, page_id: clap_id)>,
}
