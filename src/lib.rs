//! GoTiVi: deterministic Vietnamese linguistic core and transactional text actions.
//! No network I/O, global keyboard hooks or application state in this crate.
pub mod linguistic;
pub mod composer;
pub mod adapter;
pub use linguistic::{Mode,ToneStyle,Config,analyse_syllable};
pub use composer::{Composer,Event,Action,ActionKind,MAX_RAW};
pub fn convert(text:&str,mode:Mode)->String {
    linguistic::convert(text,&Config{mode,..Config::default()})
}
use std::ffi::{c_char,CStr,CString};

pub struct FfiSession {
    composer:Composer,
    text:CString,
    expected:CString,
}
#[repr(C)]
pub struct GotiviAction {
    pub kind:u32, // 0 pass, 1 replace, 2 preedit, 3 commit, 4 busy
    pub revision:u32,
    pub delete_utf16:u32,
    pub pass_key:bool,
    pub text:*const c_char,     // valid until next call to this session
    pub expected:*const c_char, // valid until next call to this session
}
fn ffi_mode(mode:u32)->Mode{
    match mode{1=>Mode::SimpleTelex,2=>Mode::Vni,3=>Mode::Viqr,4=>Mode::English,_=>Mode::Telex}
}
fn ffi_action(s:&mut FfiSession,a:Action)->GotiviAction{
    s.text=CString::new(a.insert).unwrap_or_default();
    s.expected=CString::new(a.expected).unwrap_or_default();
    GotiviAction{
        kind:match a.kind{ActionKind::Pass=>0,ActionKind::Replace=>1,ActionKind::Preedit=>2,ActionKind::Commit=>3,ActionKind::Busy=>4},
        revision:a.revision,delete_utf16:a.delete_utf16,
        pass_key:a.pass_key,text:s.text.as_ptr(),expected:s.expected.as_ptr()
    }
}
#[no_mangle]
pub extern "C" fn gotivi_new(mode:u32)->*mut FfiSession {
    let config=Config{mode:ffi_mode(mode),..Config::default()};
    Box::into_raw(Box::new(FfiSession {
        composer:Composer::new(config),
        text:CString::default(),expected:CString::default()
    }))
}
#[no_mangle]
pub unsafe extern "C" fn gotivi_free(ptr:*mut FfiSession){
    if !ptr.is_null(){drop(Box::from_raw(ptr))}
}
#[no_mangle]
pub unsafe extern "C" fn gotivi_feed(ptr:*mut FfiSession,key:*const c_char)->GotiviAction{
    let empty=GotiviAction{kind:0,revision:0,delete_utf16:0,pass_key:true,text:std::ptr::null(),expected:std::ptr::null()};
    if ptr.is_null()||key.is_null(){return empty}
    let s=&mut *ptr;
    let Ok(k)=CStr::from_ptr(key).to_str()else{return empty};
    let event=match k{
        "Backspace"=>Event::Backspace,
        "Enter"=>Event::Boundary('\n'),
        "Tab"=>Event::Boundary('\t'),
        _=>{
            let mut cs=k.chars();
            match(cs.next(),cs.next()){
                (Some(ch),None)=>Event::Key(ch),
                _=>return empty
            }
        }
    };
    let a=s.composer.feed(event);
    ffi_action(s,a)
}
#[no_mangle]
pub unsafe extern "C" fn gotivi_ack(ptr:*mut FfiSession,revision:u32,accepted:bool)->bool{
    if ptr.is_null(){return false}
    (*ptr).composer.ack(revision,accepted)
}
#[no_mangle]
pub unsafe extern "C" fn gotivi_switch_mode(ptr:*mut FfiSession,mode:u32)->GotiviAction{
    let empty=GotiviAction{kind:0,revision:0,delete_utf16:0,pass_key:true,text:std::ptr::null(),expected:std::ptr::null()};
    if ptr.is_null(){return empty}
    let s=&mut *ptr;
    let a=s.composer.switch_mode(ffi_mode(mode));
    ffi_action(s,a)
}
#[no_mangle]
pub unsafe extern "C" fn gotivi_reset(ptr:*mut FfiSession) {
    if !ptr.is_null(){(*ptr).composer.reset()}
}
#[cfg(feature="wasm")]
mod wasm{
    use super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn type_text(text:&str,mode:&str)->String {
        convert(text,Mode::parse(mode))
    }
    #[wasm_bindgen]
    pub struct WasmComposer { inner:Composer }
    #[wasm_bindgen]
    impl WasmComposer {
        #[wasm_bindgen(constructor)]
        pub fn new(mode:&str)->WasmComposer {
            WasmComposer{inner:Composer::new(Config{mode:Mode::parse(mode),..Config::default()})}
        }
        pub fn feed(&mut self,key:&str)->String{
            let event=match key {
                "Backspace"=>Event::Backspace,
                "Enter"=>Event::Boundary('\n'),
                "Tab"=>Event::Boundary('\t'),
                _ if key.chars().count()==1=>Event::Key(key.chars().next().unwrap()),
                _=>return serde_json::json!({"kind":"pass","revision":0,"pass_key":true}).to_string()
            };
            serde_json::to_string(&self.inner.feed(event)).unwrap()
        }
        pub fn ack(&mut self,revision:u32,accepted:bool)->bool{
            self.inner.ack(revision,accepted)
        }
        pub fn switch_mode(&mut self,mode:&str)->String{
            serde_json::to_string(&self.inner.switch_mode(Mode::parse(mode))).unwrap()
        }
        pub fn reset(&mut self){self.inner.reset()}
    }
}
