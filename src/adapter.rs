//! Thin host adapters. Real OS-specific drivers implement TextHost.
use crate::composer::{Action,ActionKind,Composer,Event};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Profile{Standard,Terminal,Chrome,Firefox,Game,Password}
impl Profile{
    pub fn preedit(self)->bool{matches!(self,Self::Terminal)}
    pub fn enabled(self)->bool{!matches!(self,Self::Game|Self::Password)}
}
pub trait TextHost {
    type Error;
    /// Must atomically verify expected preceding text and apply the edit,
    /// or return Err without modifying the host buffer.
    fn apply(&mut self,action:&Action)->Result<(),Self::Error>;
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum HostError{Desynchronized}
#[derive(Default)]
pub struct MemoryHost{pub text:String}
impl TextHost for MemoryHost{
    type Error=HostError;
    fn apply(&mut self,a:&Action)->Result<(),HostError>{
        match a.kind {
            ActionKind::Pass|ActionKind::Busy=>return Ok(()),
            ActionKind::Replace|ActionKind::Preedit|ActionKind::Commit=>()
        }
        if !self.text.ends_with(&a.expected){return Err(HostError::Desynchronized)}
        if a.delete_utf16!=0 &&
           a.delete_utf16!=a.expected.encode_utf16().count() as u32 {
            return Err(HostError::Desynchronized);
        }
        let new_len=self.text.len()-a.expected.len();
        self.text.truncate(new_len);
        self.text.push_str(&a.insert);
        Ok(())
    }
}
pub fn dispatch<H:TextHost>(composer:&mut Composer,host:&mut H,event:Event)->Result<Action,H::Error>{
    let action=composer.feed(event);
    if matches!(action.kind,ActionKind::Replace|ActionKind::Preedit|ActionKind::Commit){
        let applied=host.apply(&action);
        composer.ack(action.revision,applied.is_ok());
        applied?;
    } else if action.pass_key {
        // Caller is responsible for applying unhandled OS keys. MemoryHost
        // is deliberately only a reference adapter, not a keyboard hook.
    }
    Ok(action)
}
