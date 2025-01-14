use crate::version::*;

use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

pub const CLAP_PRESET_DISCOVERY_FACTORY_ID: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.preset-discovery-factory/draft-1\0") };

pub const CLAP_PRESET_DISCOVERY_IS_FACTORY_CONTENT: u32 = 1 << 0;
pub const CLAP_PRESET_DISCOVERY_IS_USER_CONTENT: u32 = 1 << 1;
pub const CLAP_PRESET_DISCOVERY_IS_DEMO_CONTENT: u32 = 1 << 2;
pub const CLAP_PRESET_DISCOVERY_IS_FAVORITE: u32 = 1 << 3;

pub type clap_timestamp = u64;

pub const CLAP_TIMESTAMP_UNKNOWN: clap_timestamp = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_id {
    pub abi: *const c_char,
    pub id: *const c_char,
}

unsafe impl Send for clap_plugin_id {}
unsafe impl Sync for clap_plugin_id {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_metadata_receiver {
    pub receiver_data: *mut c_void,
    pub on_error: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            os_error: i32,
            error_message: *const c_char,
        ),
    >,
    pub begin_preset: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            name: *const c_char,
            load_key: *const c_char,
        ) -> bool,
    >,
    pub add_plugin_id: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            plugin_id: *const clap_plugin_id,
        ),
    >,
    pub set_soundpack_id: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            soundpack_id: *const c_char,
        ),
    >,
    pub set_flags: Option<
        unsafe extern "C" fn(receiver: *const clap_preset_discovery_metadata_receiver, flags: u32),
    >,
    pub add_creator: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            creator: *const c_char,
        ),
    >,
    pub set_description: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            description: *const c_char,
        ),
    >,
    pub set_timestamps: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            creation_time: clap_timestamp,
            modification_time: clap_timestamp,
        ),
    >,
    pub add_feature: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            feature: *const c_char,
        ),
    >,
    pub add_extra_info: Option<
        unsafe extern "C" fn(
            receiver: *const clap_preset_discovery_metadata_receiver,
            key: *const c_char,
            value: *const c_char,
        ),
    >,
}

unsafe impl Send for clap_preset_discovery_metadata_receiver {}
unsafe impl Sync for clap_preset_discovery_metadata_receiver {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_filetype {
    pub name: *const c_char,
    pub description: *const c_char,
    pub file_extension: *const c_char,
}

unsafe impl Send for clap_preset_discovery_filetype {}
unsafe impl Sync for clap_preset_discovery_filetype {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_location {
    pub flags: u32,
    pub name: *const c_char,
    pub uri: *const c_char,
}

unsafe impl Send for clap_preset_discovery_location {}
unsafe impl Sync for clap_preset_discovery_location {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_soundpack {
    pub flags: u64,
    pub id: *const c_char,
    pub name: *const c_char,
    pub description: *const c_char,
    pub homepage_url: *const c_char,
    pub vendor: *const c_char,
    pub image_uri: *const c_char,
    pub release_timestamp: clap_timestamp,
}

unsafe impl Send for clap_preset_discovery_soundpack {}
unsafe impl Sync for clap_preset_discovery_soundpack {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_provider_descriptor {
    pub clap_version: clap_version,
    pub id: *const c_char,
    pub name: *const c_char,
    pub vendor: *const c_char,
}

unsafe impl Send for clap_preset_discovery_provider_descriptor {}
unsafe impl Sync for clap_preset_discovery_provider_descriptor {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_provider {
    pub desc: *const clap_preset_discovery_provider_descriptor,
    pub provider_data: *mut c_void,
    pub init: Option<unsafe extern "C" fn(provider: *const clap_preset_discovery_provider) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(provider: *const clap_preset_discovery_provider)>,
    pub get_metadata: Option<
        unsafe extern "C" fn(
            provider: *const clap_preset_discovery_provider,
            uri: *const c_char,
            metadata_receiver: *const clap_preset_discovery_metadata_receiver,
        ) -> bool,
    >,
    pub get_extension: Option<
        unsafe extern "C" fn(
            provider: *const clap_preset_discovery_provider,
            extension_id: *const c_char,
        ) -> *const c_void,
    >,
}

unsafe impl Send for clap_preset_discovery_provider {}
unsafe impl Sync for clap_preset_discovery_provider {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_indexer {
    pub clap_version: clap_version,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub url: *const c_char,
    pub version: *const c_char,
    pub indexer_data: *mut c_void,
    pub declare_filetype: Option<
        unsafe extern "C" fn(
            indexer: *const clap_preset_discovery_indexer,
            filetype: *const clap_preset_discovery_filetype,
        ) -> bool,
    >,
    pub declare_location: Option<
        unsafe extern "C" fn(
            indexer: *const clap_preset_discovery_indexer,
            location: *const clap_preset_discovery_location,
        ) -> bool,
    >,
    pub declare_soundpack: Option<
        unsafe extern "C" fn(
            indexer: *const clap_preset_discovery_indexer,
            soundpack: *const clap_preset_discovery_soundpack,
        ) -> bool,
    >,
    pub get_extension: Option<
        unsafe extern "C" fn(
            indexer: *const clap_preset_discovery_indexer,
            extension_id: *const c_char,
        ) -> *const c_void,
    >,
}

unsafe impl Send for clap_preset_discovery_indexer {}
unsafe impl Sync for clap_preset_discovery_indexer {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_preset_discovery_factory {
    pub count: Option<unsafe extern "C" fn(factory: *const clap_preset_discovery_factory) -> u32>,
    pub get_descriptor: Option<
        unsafe extern "C" fn(
            factory: *const clap_preset_discovery_factory,
            index: u32,
        ) -> *const clap_preset_discovery_provider_descriptor,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            factory: *const clap_preset_discovery_factory,
            indexer: *const clap_preset_discovery_indexer,
            provider_id: *const c_char,
        ) -> *const clap_preset_discovery_provider,
    >,
}
