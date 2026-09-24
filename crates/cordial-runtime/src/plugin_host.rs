pub fn register_static_overlays()->usize{0}
pub fn start_all()->usize{0}
pub fn start_reconciler(){}
pub fn publish_core<T>(_event:&str,_payload:T){}
pub fn flush_core_events(_:std::time::Duration)->Vec<String>{Vec::new()}
pub fn undelivered_core_events()->Vec<String>{Vec::new()}
