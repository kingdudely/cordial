use std::path::Path;
use std::sync::OnceLock;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Store{File}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Kind{Cookies,Identity}
impl Kind{pub fn name(self)->&'static str{match self{Self::Cookies=>"cookies",Self::Identity=>"identity"}}}
#[derive(Clone,Copy)]pub struct SnapshotRequest<'a>{pub store:Store,pub profile_dir:&'a Path,pub kind:Kind}
pub fn active()->Store{static S:OnceLock<Store>=OnceLock::new();*S.get_or_init(||Store::File)}
pub fn load(_s:Store,d:&Path,k:Kind)->Option<String>{std::fs::read_to_string(d.join(k.name())).ok()}
pub fn save(_s:Store,d:&Path,k:Kind,b:&str)->std::io::Result<()>{std::fs::create_dir_all(d)?;let p=d.join(k.name());let tmp=d.join(format!(".{}.tmp",k.name()));std::fs::write(&tmp,b)?;std::fs::set_permissions(&tmp,std::os::unix::fs::PermissionsExt::from_mode(0o600))?;std::fs::rename(tmp,p)}
pub fn erase(_s:Store,d:&Path,k:Kind)->std::io::Result<()>{match std::fs::remove_file(d.join(k.name())){Ok(())|Err(_)=>Ok(())}}
pub fn where_kept(_s:Store,d:&Path,k:Kind)->String{d.join(k.name()).display().to_string()}
pub fn usable()->Result<(),String>{Ok(())}
pub fn snapshot(_r:SnapshotRequest<'_>)->Option<String>{None}
