use std::ffi::c_void;
#[derive(Default,Clone)]pub struct Vocabulary;
#[derive(Default,Clone)]pub struct BusNatives;
#[derive(Default,Clone)]pub struct OpenWindowRequest{pub url:String}
pub fn read_vocabulary(_:impl FnMut(&str)->Option<*mut c_void>)->Vocabulary{Vocabulary}
pub fn report(_: &Vocabulary){}
pub fn find_bus_natives(_:impl FnMut(&str)->Option<*mut c_void>)->BusNatives{BusNatives}
pub fn report_bus_natives(_: &BusNatives){}
pub fn user_agent()->Option<String>{None}
pub fn roblox_session_cookie(_: &std::path::Path)->Option<String>{None}
pub fn set_presenter(_:impl Fn(OpenWindowRequest)+Send+Sync+'static){}
pub fn set_close_handler(_:impl Fn()+Send+Sync+'static){}
pub fn dev_trigger_open_window(_:String){}
pub fn trace_bridge()->bool{false}
pub fn arm(_:impl FnMut(&str)->Option<*mut c_void>){}
pub fn report_window_closed(){}
pub fn hybrid_launch_enabled()->bool{false}
pub fn forward_bridge_message(_: &str){}
pub fn connected()->Option<bool>{Some(false)}
pub fn to_shell_request(_: &OpenWindowRequest,_:Option<String>){}
