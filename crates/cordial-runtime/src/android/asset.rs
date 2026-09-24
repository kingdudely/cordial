use std::ffi::{c_char,c_int,c_void,CStr};
use std::path::{Path,PathBuf};
use std::sync::{Mutex,OnceLock};
struct Asset{bytes:&'static [u8]}
struct Manager{root:PathBuf,cache:Mutex<std::collections::HashMap<String,&'static [u8]>>}
static MANAGER:OnceLock<Manager>=OnceLock::new();
static REQUESTED:Mutex<Vec<String>>=Mutex::new(Vec::new());
pub fn set_asset_dir(p:&Path)->Result<(),String>{let r=p.canonicalize().map_err(|e|format!("{}: {e}",p.display()))?;if !r.is_dir(){return Err(format!("{} is not a directory",r.display()))}MANAGER.set(Manager{root:r,cache:Mutex::new(std::collections::HashMap::new())}).map_err(|_|"asset directory already configured".into())}
pub fn set_asset_root(p:&Path){let _=set_asset_dir(p)}
pub fn set_apk(_:&Path)->Result<(),String>{Err("APK input is disabled; provide assets/ beside roblox".into())}
pub fn is_configured()->bool{MANAGER.get().is_some()}
fn valid(n:&str)->bool{let p=Path::new(n);!p.is_absolute()&&!p.components().any(|c|matches!(c,std::path::Component::ParentDir))}
fn read(n:&str)->Option<&'static [u8]>{if !valid(n){return None}let m=MANAGER.get()?;if let Ok(c)=m.cache.lock(){if let Some(b)=c.get(n){return Some(b)}}let b=std::fs::read(m.root.join(n)).ok()?;let b:&'static [u8]=Box::leak(b);if let Ok(mut c)=m.cache.lock(){c.insert(n.to_string(),b);}if let Ok(mut q)=REQUESTED.lock(){q.push(n.to_string());}Some(b)}
extern "C" fn from_java(_: *mut c_void,_:*mut c_void)->*mut c_void{MANAGER.get().map_or(std::ptr::null_mut(),|m|m as *const _ as *mut c_void)}
extern "C" fn open(_: *mut c_void,p:*const c_char,_:c_int)->*mut c_void{let Some(s)=(unsafe{(!p.is_null()).then(||CStr::from_ptr(p))}).and_then(|x|x.to_str().ok())else{return std::ptr::null_mut()};read(s).map_or(std::ptr::null_mut(),|b|Box::into_raw(Box::new(Asset{bytes:b})) as *mut c_void)}
extern "C" fn len(a:*mut c_void)->i64{if a.is_null(){0}else{unsafe{(*(a as *const Asset)).bytes.len() as i64}}}
extern "C" fn buf(a:*mut c_void)->*const c_void{if a.is_null(){std::ptr::null()}else{unsafe{(*(a as *const Asset)).bytes.as_ptr() as *const c_void}}}
extern "C" fn close(a:*mut c_void){if !a.is_null(){unsafe{drop(Box::from_raw(a as *mut Asset))}}}
extern "C" fn fd(a:*mut c_void,start:*mut i64,out_len:*mut i64)->c_int{if a.is_null(){return -1}extern "C"{fn memfd_create(*const c_char,u32)->c_int;fn write(c_int,*const c_void,usize)->isize;fn lseek(c_int,i64,c_int)->i64}let b=unsafe{(*(a as *const Asset)).bytes};let x=unsafe{memfd_create(c"roblox-asset".as_ptr(),0)};if x<0{return -1}let n=unsafe{write(x,b.as_ptr() as *const c_void,b.len())};if n<0{return -1}unsafe{lseek(x,0,0)};if !start.is_null(){unsafe{*start=0}}if !out_len.is_null(){unsafe{*out_len=b.len() as i64}}x}
pub fn overrides()->Vec<(&'static str,*mut c_void)>{macro_rules!f{($n:literal,$f:expr)=>{($n,$f as *const () as *mut c_void)}}vec![f!("AAssetManager_fromJava",from_java),f!("AAssetManager_open",open),f!("AAsset_getBuffer",buf),f!("AAsset_getLength",len),f!("AAsset_close",close),f!("AAsset_openFileDescriptor",fd)]}
pub fn probe(n:&str)->Result<usize,String>{read(n).map(|b|b.len()).ok_or_else(||format!("asset not found: {n}"))}
pub fn trace_path()->PathBuf{crate::profile::active().join("asset-trace.log")}
pub fn write_trace(p:&Path)->std::io::Result<usize>{let mut q=REQUESTED.lock().unwrap();q.sort();q.dedup();std::fs::write(p,q.join("\n"))?;Ok(q.len())}
pub fn shadow_report()->Vec<String>{Vec::new()}
pub fn start_watcher()->bool{false}
pub fn index()->Index{Index}
pub struct Index;
impl Index{pub fn len(&self)->usize{0}pub fn is_empty(&self)->bool{true}}
