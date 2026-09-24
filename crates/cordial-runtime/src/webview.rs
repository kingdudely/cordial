use std::ffi::c_void;

#[derive(Default, Clone)]
pub struct Vocabulary;

#[derive(Default, Clone)]
pub struct BusNatives;

#[derive(Default, Clone)]
pub struct OpenWindowRequest {
    pub url: String,
}

pub fn read_vocabulary(
    _find_symbol: impl FnMut(&str) -> Option<*mut c_void>,
) -> Vocabulary {
    Vocabulary
}

pub fn report(_vocabulary: &Vocabulary) {}

pub fn find_bus_natives(
    _find_symbol: impl FnMut(&str) -> Option<*mut c_void>,
) -> BusNatives {
    BusNatives
}

pub fn report_bus_natives(_natives: &BusNatives) {}

pub fn user_agent() -> Option<String> {
    None
}

pub fn roblox_session_cookie(_profile: &std::path::Path) -> Option<String> {
    None
}

pub fn set_presenter(
    _presenter: impl Fn(OpenWindowRequest) + Send + Sync + 'static,
) {
}

pub fn set_close_handler(_handler: impl Fn() + Send + Sync + 'static) {}

pub fn dev_trigger_open_window(_url: String) {}

pub fn trace_bridge() -> bool {
    false
}

pub fn arm(_find_symbol: impl FnMut(&str) -> Option<*mut c_void>) {}

pub fn report_window_closed() {}

pub fn hybrid_launch_enabled() -> bool {
    false
}

pub fn forward_bridge_message(_message: &str) {}

pub fn connected() -> Option<bool> {
    Some(false)
}

pub fn to_shell_request(
    _request: &OpenWindowRequest,
    _cookie: Option<String>,
) {
}
