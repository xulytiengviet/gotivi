use gotivi::{Config,Mode,ToneStyle,convert,linguistic};
fn cv(s:&str,mode:Mode)->String{convert(s,mode)}
#[test] fn telex_examples(){
    let cases=[
        ("Tieengs Vieejt","Tiếng Việt"),
        ("thuowng","thương"),
        ("dd","đ"),
        ("DD","Đ"),
        ("too","tô"),
        ("mix","mĩ"),
        ("mixx","mix"),
        ("ko vn","không Việt Nam"),
        ("aaf","ầ"),
        ("uow","ươ"),
    ];
    for (raw,want) in cases{assert_eq!(cv(raw,Mode::Telex),want,"raw={raw}");}
}
#[test] fn other_methods(){
    assert_eq!(cv("tie6ng1",Mode::Vni),"tiếng");
    assert_eq!(cv("tie^'ng",Mode::Viqr),"tiếng");
    assert_eq!(cv("dd",Mode::Viqr),"đ");
    assert_eq!(cv("tieengs",Mode::SimpleTelex),"tiếng");
    assert_eq!(cv("Tieengs",Mode::English),"Tieengs");
}
#[test] fn tone_style(){
    let mut c=Config::default();
    c.tone_style=ToneStyle::Modern;
    assert_eq!(linguistic::compose("hoaf",&c),"hòa");
    c.tone_style=ToneStyle::Traditional;
    assert_eq!(linguistic::compose("hoaf",&c),"hoà");
}
#[test] fn macros_are_only_applied_on_finish(){
    let c=Config::default();
    assert_eq!(linguistic::compose("ko",&c),"ko");
    assert_eq!(linguistic::finish("ko",&c),"không");
    assert_eq!(linguistic::finish("VN",&c),"VIỆT NAM");
}
#[test] fn unicode_normalization(){
    let input="e\u{301}";  // decomposed e acute
    assert_eq!(cv(input,Mode::English),"é");
}
#[test] fn shape_is_heuristic_not_dictionary(){
    let shape=linguistic::analyse_syllable("thương");
    assert_eq!(shape.onset,"th");
    assert_eq!(shape.nucleus,"ươ");
    assert_eq!(shape.coda,"ng");
    assert!(shape.plausible);
}
