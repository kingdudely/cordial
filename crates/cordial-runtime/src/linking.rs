pub fn arm(_handler: impl Fn(&str)+Send+Sync+'static){}
pub fn on_open_url(url:&str){if url.starts_with("http://")||url.starts_with("https://"){let _=std::process::Command::new("xdg-open").arg(url).spawn();}}
