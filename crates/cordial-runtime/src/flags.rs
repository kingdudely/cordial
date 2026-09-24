use std::collections::BTreeMap;
use std::path::{Path,PathBuf};
#[derive(Clone,Debug)]pub enum Source{User,System}
impl Source{pub fn describe(&self)->String{match self{Self::User=>"user".into(),Self::System=>"system".into()}}}
#[derive(Clone,Debug)]pub struct Resolved{pub value:String,pub source:Source}
#[derive(Clone,Debug)]pub struct Layer{pub source:Source,pub values:BTreeMap<String,String>}
pub fn user_path_in(d:&Path)->PathBuf{d.join("flags.json")}
pub fn user_path()->PathBuf{user_path_in(&crate::profile::active())}
pub fn parse_user_text(t:&str)->Result<BTreeMap<String,String>,String>{let v:serde_json::Value=serde_json::from_str(t).map_err(|e|e.to_string())?;let o=v.as_object().ok_or("flags.json must be an object")?;Ok(o.iter().map(|(k,v)|(k.clone(),match v{serde_json::Value::String(s)=>s.clone(),_=>v.to_string()})).collect())}
pub fn read_layer(p:&Path,s:Source)->Option<Layer>{let t=std::fs::read_to_string(p).ok()?;Some(Layer{source:s,values:parse_user_text(&t).ok()?})}
pub fn resolve(l:&[Layer])->BTreeMap<String,Resolved>{let mut o=BTreeMap::new();for x in l{for(k,v)in &x.values{o.insert(k.clone(),Resolved{value:v.clone(),source:x.source.clone()});}}o}
pub fn collect()->Vec<Layer>{read_layer(&user_path(),Source::User).into_iter().collect()}
pub fn write_user_layer(d:&Path,v:&BTreeMap<String,String>)->Result<(),String>{std::fs::create_dir_all(d).map_err(|e|e.to_string())?;let mut o=serde_json::Map::new();for(k,x)in v{o.insert(k.clone(),serde_json::Value::String(x.clone()));}std::fs::write(user_path_in(d),serde_json::to_string_pretty(&o).map_err(|e|e.to_string())?).map_err(|e|e.to_string())}
pub fn report(_:&BTreeMap<String,Resolved>){}
