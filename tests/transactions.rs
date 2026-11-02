use gotivi::{Composer,Config,Event,Mode,ActionKind,MAX_RAW};
use gotivi::adapter::{MemoryHost,TextHost,dispatch,HostError};
#[test]fn transactional_unicode_and_commit(){
    let mut c=Composer::new(Config::default());
    let mut host=MemoryHost::default();
    for ch in "Tieengs".chars(){dispatch(&mut c,&mut host,Event::Key(ch)).unwrap();}
    assert_eq!(host.text,"Tiếng");
    assert_eq!(c.current(),"Tiếng");
    dispatch(&mut c,&mut host,Event::Boundary(' ')).unwrap();
    assert_eq!(host.text,"Tiếng ");
    assert_eq!(c.raw(),"");
    assert_eq!(c.current(),"");
}
#[test]fn backspace_restores_previous_raw_state(){
    let mut c=Composer::new(Config::default());let mut h=MemoryHost::default();
    for ch in "tieengs".chars(){dispatch(&mut c,&mut h,Event::Key(ch)).unwrap();}
    assert_eq!(h.text,"tiếng");
    dispatch(&mut c,&mut h,Event::Backspace).unwrap();
    assert_eq!(h.text,"tiêng");
    assert_eq!(c.raw(),"tieeng");
}
#[test]fn ack_must_match_and_rejections_force_reset(){
    let mut c=Composer::new(Config::default());
    let first=c.feed(Event::Key('a'));
    assert_eq!(first.kind,ActionKind::Replace);
    assert_eq!(c.feed(Event::Key('b')).kind,ActionKind::Busy);
    assert!(!c.ack(first.revision+1,true));
    assert!(c.ack(first.revision,false));
    assert!(c.is_desynced());
    assert_eq!(c.feed(Event::Key('c')).kind,ActionKind::Busy);
    c.reset();
    assert_eq!(c.feed(Event::Key('c')).kind,ActionKind::Replace);
}
#[test]fn host_detects_mismatch_without_deleting_data(){
    let mut c=Composer::new(Config::default());let mut h=MemoryHost::default();
    let a=c.feed(Event::Key('a'));
    h.text="unrelated".to_string();
    assert_eq!(h.apply(&a),Err(HostError::Desynchronized));
    assert_eq!(h.text,"unrelated");
    assert!(c.ack(a.revision,false));
}
#[test]fn macro_fires_at_boundary_not_during_word(){
    let mut c=Composer::new(Config::default());let mut h=MemoryHost::default();
    for ch in "ko".chars(){dispatch(&mut c,&mut h,Event::Key(ch)).unwrap();}
    assert_eq!(h.text,"ko");
    dispatch(&mut c,&mut h,Event::Boundary(' ')).unwrap();
    assert_eq!(h.text,"không ");
}
#[test]fn overflow_never_discards_host_text(){
    let mut c=Composer::new(Config::default());let mut h=MemoryHost::default();
    for ch in std::iter::repeat('b').take(MAX_RAW+2){
        dispatch(&mut c,&mut h,Event::Key(ch)).unwrap();
    }
    assert_eq!(h.text,"b".repeat(MAX_RAW+2));
}
#[test]fn switch_mode_is_transactional(){
    let mut c=Composer::new(Config::default());let mut h=MemoryHost::default();
    for ch in "to".chars(){dispatch(&mut c,&mut h,Event::Key(ch)).unwrap();}
    let a=c.switch_mode(Mode::English);
    if a.kind==ActionKind::Replace {h.apply(&a).unwrap();c.ack(a.revision,true);}
    dispatch(&mut c,&mut h,Event::Key('o')).unwrap();
    assert_eq!(h.text,"too");
}
