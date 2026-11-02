//! Transactional composer: no host I/O, one revisioned action per key.
use serde::Serialize;
use crate::linguistic::{compose,finish,is_key,Config,Mode};
pub const MAX_RAW:usize=24;
#[derive(Debug,Clone,Copy)]
pub enum Event{Key(char),Boundary(char),Backspace}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize)]
#[serde(rename_all="snake_case")]
pub enum ActionKind{Pass,Replace,Preedit,Commit,Busy}
#[derive(Debug,Clone,Serialize)]
pub struct Action{
    pub kind:ActionKind,
    pub revision:u32,
    pub delete_utf16:u32,
    pub expected:String,
    pub insert:String,
    pub pass_key:bool
}
#[derive(Debug,Clone)]
struct State{raw:String,display:String,config:Config}
struct Pending{revision:u32,state:State}
pub struct Composer{
    state:State,pending:Option<Pending>,revision:u32,
    preedit:bool,desynced:bool
}
impl Composer{
    pub fn new(config:Config)->Self{
        Self{state:State{raw:String::new(),display:String::new(),config},pending:None,revision:0,preedit:false,desynced:false}
    }
    pub fn with_preedit(mut self,enabled:bool)->Self{self.preedit=enabled;self}
    pub fn current(&self)->&str{&self.state.display}
    pub fn raw(&self)->&str{&self.state.raw}
    pub fn mode(&self)->Mode{self.state.config.mode}
    pub fn revision(&self)->u32{self.revision}
    pub fn reset(&mut self){self.state.raw.clear();self.state.display.clear();self.pending=None;self.desynced=false;self.revision=self.revision.wrapping_add(1);}
    fn passive(&self,pass_key:bool,kind:ActionKind)->Action{
        Action{kind,revision:self.revision,delete_utf16:0,expected:String::new(),insert:String::new(),pass_key}
    }
    fn schedule(&mut self,next:State,insert:String,kind:ActionKind)->Action{
        let old=self.state.display.clone();
        if insert==old && kind!=ActionKind::Commit{
            self.state=next;
            return self.passive(false,ActionKind::Pass);
        }
        let id=self.revision.wrapping_add(1);
        let action=Action{
            kind,revision:id,
            delete_utf16:if self.preedit {0}else{old.encode_utf16().count() as u32},
            expected:old,insert,pass_key:false
        };
        self.pending=Some(Pending{revision:id,state:next});
        action
    }
    pub fn switch_mode(&mut self,mode:Mode)->Action{
        if self.pending.is_some()||self.desynced{return self.passive(false,ActionKind::Busy)}
        let mut next=self.state.clone();next.config.mode=mode;
        next.display=compose(&next.raw,&next.config);
        let value=next.display.clone();
        self.schedule(next,value,if self.preedit{ActionKind::Preedit}else{ActionKind::Replace})
    }
    pub fn feed(&mut self,event:Event)->Action{
        if self.pending.is_some()||self.desynced{return self.passive(false,ActionKind::Busy)}
        let mut next=self.state.clone();
        match event {
            Event::Backspace=>{
                if next.raw.is_empty(){return self.passive(true,ActionKind::Pass)}
                next.raw.pop();
                next.display=compose(&next.raw,&next.config);
                let value=next.display.clone();
                self.schedule(next,value,if self.preedit{ActionKind::Preedit}else{ActionKind::Replace})
            }
            Event::Boundary(ch)=>{
                if next.raw.is_empty(){return self.passive(true,ActionKind::Pass)}
                let committed=finish(&next.raw,&next.config);
                next.raw.clear();next.display.clear();
                let value=format!("{committed}{ch}");
                self.schedule(next,value,ActionKind::Commit)
            }
            Event::Key(ch)=>{
                if !ch.is_ascii()||!is_key(ch,next.config.mode){
                    return self.feed(Event::Boundary(ch))
                }
                if next.raw.len()>=MAX_RAW{
                    // Atomically finalize the previous bounded word and begin a new one.
                    let committed=finish(&next.raw,&next.config);
                    next.raw.clear();next.raw.push(ch);
                    next.display=compose(&next.raw,&next.config);
                    let value=format!("{committed}{}",next.display);
                    return self.schedule(next,value,ActionKind::Commit);
                }
                next.raw.push(ch);
                next.display=compose(&next.raw,&next.config);
                let value=next.display.clone();
                self.schedule(next,value,if self.preedit{ActionKind::Preedit}else{ActionKind::Replace})
            }
        }
    }
    pub fn ack(&mut self,revision:u32,accepted:bool)->bool{
        let Some(pending)=self.pending.take() else{return false};
        if pending.revision!=revision{
            self.pending=Some(pending);return false
        }
        if accepted{self.state=pending.state;self.revision=revision;}
        else{self.desynced=true;self.state.raw.clear();self.state.display.clear();}
        true
    }
    pub fn is_desynced(&self)->bool{self.desynced}
}
