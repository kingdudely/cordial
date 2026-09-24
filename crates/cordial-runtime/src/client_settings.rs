use std::path::{Path,PathBuf};
use std::time::{Duration,SystemTime};
pub enum Source{Explicit,Cache,Fetched,Nothing(String)}
impl std::fmt::Display for Source{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Explicit=>write!(f,"explicit"),Self::Cache=>write!(f,"cache"),Self::Fetched=>write!(f,"fetched"),Self::Nothing(s)=>write!(f,"nothing: {s}")}}}
const URL:&str="https://clientsettingscdn.roblox.com/v2/settings-compressed/application/GoogleAndroidApp.zst";
fn cache_path()->PathBuf{crate::profile::active().join("clientsettings.json")}
fn fresh(p:&Path)->Option<String>{let m=std::fs::metadata(p).ok()?;let age=SystemTime::now().duration_since(m.modified().ok()?).ok()?;(age<Duration::from_secs(21600)).then(||std::fs::read_to_string(p).ok()).flatten()}
fn fetch()->Result<String,String>{let mut r=ureq::get(URL).call().map_err(|e|e.to_string())?;let b=r.body_mut().read_to_vec().map_err(|e|e.to_string())?;String::from_utf8(zstd::stream::decode_all(std::io::Cursor::new(b)).map_err(|e|e.to_string())?).map_err(|e|e.to_string())}
pub fn load(p:Option<&str>)->Option<String>{load_reporting(p).0}
pub fn load_reporting(p:Option<&str>)->(Option<String>,Source){if let Some(p)=p{return match std::fs::read_to_string(p){Ok(v)=>(Some(v),Source::Explicit),Err(e)=>(None,Source::Nothing(e.to_string()))}}let c=cache_path();if let Some(v)=fresh(&c){return(Some(v),Source::Cache)}match fetch(){Ok(v)=>{let _=std::fs::create_dir_all(c.parent().unwrap());let _=std::fs::write(&c,&v);(Some(v),Source::Fetched)},Err(e)=>(None,Source::Nothing(e))}}
