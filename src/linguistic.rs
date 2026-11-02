//! Stateless, deterministic Vietnamese orthographic composer.
use unicode_normalization::UnicodeNormalization;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Mode{Telex,SimpleTelex,Vni,Viqr,English}
impl Mode{pub fn parse(s:&str)->Self{match s.to_ascii_lowercase().as_str(){"simple"|"simple-telex"=>Self::SimpleTelex,"vni"=>Self::Vni,"viqr"=>Self::Viqr,"en"|"english"=>Self::English,_=>Self::Telex}}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ToneStyle{Modern,Traditional}
#[derive(Debug,Clone)]
pub struct Config{pub mode:Mode,pub tone_style:ToneStyle,pub quick_telex:bool,pub macros:bool}
impl Default for Config{fn default()->Self{Self{mode:Mode::Telex,tone_style:ToneStyle::Modern,quick_telex:true,macros:true}}}
const VOWELS:[&str;12]=["aáàảãạ","ăắằẳẵặ","âấầẩẫậ","eéèẻẽẹ","êếềểễệ","iíìỉĩị","oóòỏõọ","ôốồổỗộ","ơớờởỡợ","uúùủũụ","ưứừửữự","yýỳỷỹỵ"];
fn vowel(c:char)->Option<(usize,usize)>{let x=c.to_lowercase().next()?;VOWELS.iter().enumerate().find_map(|(i,s)|s.chars().position(|y|x==y).map(|j|(i,j)))}
fn make(old:char,row:usize,tone:usize)->char{let c=VOWELS[row].chars().nth(tone).unwrap();if old.is_uppercase(){c.to_uppercase().next().unwrap()}else{c}}
fn transform_last(v:&mut[char],pairs:&[(usize,usize)])->bool{if let Some(c)=v.last_mut(){if let Some((row,t))=vowel(*c){if let Some((_,target))=pairs.iter().find(|(from,_)|*from==row){*c=make(*c,*target,t);return true}}}false}
fn transform_vowel(v:&mut[char],pairs:&[(usize,usize)])->bool{for c in v.iter_mut().rev(){if let Some((row,t))=vowel(*c){if let Some((_,target))=pairs.iter().find(|(from,_)|*from==row){*c=make(*c,*target,t);return true}}}false}
fn apply_w(v:&mut Vec<char>)->bool{let n=v.len();if n>=2&&vowel(v[n-2]).is_some_and(|(r,_)|r==9)&&vowel(v[n-1]).is_some_and(|(r,_)|r==6){let t1=vowel(v[n-2]).unwrap().1;let t2=vowel(v[n-1]).unwrap().1;v[n-2]=make(v[n-2],10,t1);v[n-1]=make(v[n-1],8,t2);true}else{transform_last(v,&[(0,1),(6,8),(9,10)])}}
fn tone_key(c:char,mode:Mode)->Option<usize>{match mode{Mode::Telex|Mode::SimpleTelex=>match c.to_ascii_lowercase(){'s'=>Some(1),'f'=>Some(2),'r'=>Some(3),'x'=>Some(4),'j'=>Some(5),'z'=>Some(0),_=>None},Mode::Vni=>match c{'1'=>Some(1),'2'=>Some(2),'3'=>Some(3),'4'=>Some(4),'5'=>Some(5),'0'=>Some(0),_=>None},Mode::Viqr=>match c{'\''=>Some(1),'\x60'=>Some(2),'?'=>Some(3),'~'=>Some(4),'.'=>Some(5),_=>None},Mode::English=>None}}
fn set_tone(v:&mut[char],t:usize,style:ToneStyle){if t==0{return}let mut ps:Vec<usize>=v.iter().enumerate().filter_map(|(i,c)|vowel(*c).map(|_|i)).collect();if ps.is_empty(){return}
if ps.len()>1&&v.len()>2&&v[0].eq_ignore_ascii_case(&'g')&&v[1].eq_ignore_ascii_case(&'i'){ps.retain(|p|*p!=1)}
if ps.len()>1&&v.len()>2&&v[0].eq_ignore_ascii_case(&'q')&&v[1].eq_ignore_ascii_case(&'u'){ps.retain(|p|*p!=1)}
if ps.is_empty(){return}let last=*ps.last().unwrap();let mut pick=last;
if let Some(&i)=ps.iter().find(|&&i|matches!(vowel(v[i]),Some((4|7|8,_)))){pick=i}
else if ps.len()>1{let first=ps[0];let a=vowel(v[first]).unwrap().0;let b=vowel(v[last]).unwrap().0;let coda=last+1<v.len();
if !coda&&matches!((a,b),(6,0)|(6,3)|(9,11)){pick=if style==ToneStyle::Traditional{first}else{last}}
else{pick=if coda{last}else{first}}}
let(row,_)=vowel(v[pick]).unwrap();v[pick]=make(v[pick],row,t);}
fn case_like(raw:&str,replace:&str)->String{if raw.chars().all(|c|c.is_uppercase()){replace.to_uppercase()}else if raw.chars().next().is_some_and(|c|c.is_uppercase()){let mut it=replace.chars();format!("{}{}",it.next().unwrap().to_uppercase(),it.collect::<String>())}else{replace.into()}}
pub fn compose(raw:&str,config:&Config)->String{if config.mode==Mode::English{return raw.nfc().collect()}let mut v:Vec<char>=Vec::new();let mut tone=0usize;let mut previous_tone=None;let mut escaped=false;
for ch in raw.chars(){let low=ch.to_ascii_lowercase();if escaped{v.push(ch);continue}
if let Some(mark)=tone_key(ch,config.mode){if v.iter().any(|c|vowel(*c).is_some()){if previous_tone==Some(low)&&tone==mark&&mark!=0{tone=0;v.push(ch);escaped=true;continue}tone=mark;previous_tone=Some(low);continue}}
previous_tone=None;
let consumed=match config.mode{
Mode::Telex|Mode::SimpleTelex=>match low{'a'=>transform_last(&mut v,&[(0,2)]),'e'=>transform_last(&mut v,&[(3,4)]),'o'=>transform_last(&mut v,&[(6,7)]),'w'=>apply_w(&mut v),'d' if v.last().is_some_and(|x|x.eq_ignore_ascii_case(&'d'))=>{let x=v.pop().unwrap();v.push(if x.is_uppercase(){'Đ'}else{'đ'});true},_=>false},
Mode::Vni=>match low{'6'=>transform_vowel(&mut v,&[(0,2),(3,4),(6,7)]),'7'=>{if v.len()>=2&&vowel(v[v.len()-2]).is_some_and(|(g,_)|g==9)&&vowel(v[v.len()-1]).is_some_and(|(g,_)|g==6){apply_w(&mut v)}else{transform_vowel(&mut v,&[(6,8),(9,10)])}},'8'=>transform_vowel(&mut v,&[(0,1)]),'9' if v.last().is_some_and(|x|x.eq_ignore_ascii_case(&'d'))=>{let x=v.pop().unwrap();v.push(if x.is_uppercase(){'Đ'}else{'đ'});true},_=>false},
Mode::Viqr=>match low{'^'=>transform_vowel(&mut v,&[(0,2),(3,4),(6,7)]),'('=>transform_vowel(&mut v,&[(0,1)]),'+'=>transform_vowel(&mut v,&[(6,8),(9,10)]),'d' if v.last().is_some_and(|x|x.eq_ignore_ascii_case(&'d'))=>{let x=v.pop().unwrap();v.push(if x.is_uppercase(){'Đ'}else{'đ'});true},_=>false},Mode::English=>false};
if consumed{continue}
if matches!(config.mode,Mode::Telex|Mode::SimpleTelex)&&config.quick_telex{if ch=='['{v.push('ư');continue}if ch==']'{v.push('ơ');continue}if low=='w'&&config.mode==Mode::Telex&&!v.iter().any(|c|vowel(*c).is_some()){v.push(if ch.is_uppercase(){'Ư'}else{'ư'});continue}}
v.push(ch)}
set_tone(&mut v,tone,config.tone_style);v.iter().collect::<String>().nfc().collect()}
pub fn finish(raw:&str,config:&Config)->String{if config.macros&&config.mode!=Mode::English{match raw.to_ascii_lowercase().as_str(){"ko"=>return case_like(raw,"không"),"vn"=>return case_like(raw,"Việt Nam"),_=>()}}compose(raw,config)}
pub fn is_key(c:char,mode:Mode)->bool{if c.is_ascii_alphabetic(){return true}match mode{Mode::Telex|Mode::SimpleTelex=>c=='['||c==']',Mode::Vni=>c.is_ascii_digit(),Mode::Viqr=>matches!(c,'^'|'('|'+'|'\''|'\x60'|'?'|'~'|'.'),Mode::English=>c.is_ascii_alphanumeric()}}
pub fn convert(text:&str,config:&Config)->String{let mut output=String::new();let mut raw=String::new();for c in text.chars(){if is_key(c,config.mode){if raw.len()>=24{output.push_str(&finish(&raw,config));raw.clear()}raw.push(c)}else{output.push_str(&finish(&raw,config));raw.clear();output.push(c)}}output.push_str(&finish(&raw,config));output.nfc().collect()}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct SyllableShape{pub onset:String,pub nucleus:String,pub coda:String,pub plausible:bool}
pub fn analyse_syllable(text:&str)->SyllableShape{let cs:Vec<char>=text.chars().collect();let positions:Vec<usize>=cs.iter().enumerate().filter_map(|(i,c)|vowel(*c).map(|_|i)).collect();if positions.is_empty(){return SyllableShape{onset:text.into(),nucleus:String::new(),coda:String::new(),plausible:false}}let start=positions[0];let end=positions[positions.len()-1]+1;let onset=cs[..start].iter().collect();let nucleus=cs[start..end].iter().collect();let coda:String=cs[end..].iter().collect();let endings=["","m","n","ng","nh","p","t","c","ch"];let plausible=endings.contains(&coda.to_ascii_lowercase().as_str())&&cs.len()<=12;SyllableShape{onset,nucleus,coda,plausible}}
