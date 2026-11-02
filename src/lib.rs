use std::ffi::{CStr,CString,c_char};
pub const MAX_RAW:usize=24;
const LETTERS:&str="aáàảãạăắằẳẵặâấầẩẫậeéèẻẽẹêếềểễệiíìỉĩịoóòỏõọôốồổỗộơớờởỡợuúùủũụưứừửữựyýỳỷỹỵ";
const GROUPS:[&str;12]=["aáàảãạ","ăắằẳẵặ","âấầẩẫậ","eéèẻẽẹ","êếềểễệ","iíìỉĩị","oóòỏõọ","ôốồổỗộ","ơớờởỡợ","uúùủũụ","ưứừửữự","yýỳỷỹỵ"];
fn vowel(c:char)->Option<(usize,usize)> {let c=c.to_lowercase().next()?;GROUPS.iter().enumerate().find_map(|(i,s)|s.chars().position(|x|x==c).map(|j|(i,j)))}
fn make(c:char,g:usize,t:usize)->char {let r=GROUPS[g].chars().nth(t).unwrap();if c.is_uppercase(){r.to_uppercase().next().unwrap()}else{r}}
#[derive(Clone,Copy,PartialEq,Eq)]pub enum Mode{Telex,Simple,Vni,Viqr,English}
impl Mode{pub fn from(s:&str)->Self{match s.to_lowercase().as_str(){"simple"|"simple-telex"=>Self::Simple,"vni"=>Self::Vni,"viqr"=>Self::Viqr,"en"|"english"=>Self::English,_=>Self::Telex}}}
fn swap(v:&mut[char],pairs:&[(usize,usize)])->bool {for c in v.iter_mut().rev(){if let Some((g,t))=vowel(*c){if let Some((_,new))=pairs.iter().find(|(old,_)|*old==g){*c=make(*c,*new,t);return true}}}false}
pub fn compose(raw:&str,mode:Mode)->String{
 if mode==Mode::English{return raw.into()}
 if raw.eq_ignore_ascii_case("ko"){return case(raw,"không")}
 if raw.eq_ignore_ascii_case("vn"){return case(raw,"Việt Nam")}
 let mut chars=Vec::<char>::new();let mut tone=0usize;let mut prev='\0';
 for c in raw.chars(){let x=c.to_ascii_lowercase();let t=match mode{
 Mode::Telex|Mode::Simple=>match x{'s'=>1,'f'=>2,'r'=>3,'x'=>4,'j'=>5,'z'=>6,_=>0},
 Mode::Vni=>match x{'1'=>1,'2'=>2,'3'=>3,'4'=>4,'5'=>5,'0'=>6,_=>0},
 Mode::Viqr=>match x{'\''=>1,'\x60'=>2,'?'=>3,'~'=>4,'.'=>5,_=>0},_=>0};
 if t>0&&chars.iter().any(|c|vowel(*c).is_some()){tone=if t==6{0}else{t};prev=x;continue;}
 let modified=match mode{
 Mode::Telex|Mode::Simple=>match x{'a'=>swap(&mut chars,&[(0,2)]),'e'=>swap(&mut chars,&[(3,4)]),'o'=>swap(&mut chars,&[(6,7)]),'w'=>swap(&mut chars,&[(0,1),(6,8),(9,10)]),_=>false},
 Mode::Vni=>match x{'6'=>swap(&mut chars,&[(0,2),(3,4),(6,7)]),'7'=>swap(&mut chars,&[(6,8),(9,10)]),'8'=>swap(&mut chars,&[(0,1)]),_=>false},
 Mode::Viqr=>match x{'^'=>swap(&mut chars,&[(0,2),(3,4),(6,7)]),'+'=>swap(&mut chars,&[(6,8),(9,10)]),'('=>swap(&mut chars,&[(0,1)]),_=>false},_=>false};
 if modified{prev=x;continue}
 if (mode==Mode::Telex||mode==Mode::Simple)&&x=='d'&&chars.last().map(|c|c.to_ascii_lowercase())==Some('d'){let old=chars.pop().unwrap();chars.push(if old.is_uppercase(){'Đ'}else{'đ'});prev=x;continue}
 if mode==Mode::Vni&&x=='9'&&chars.last().map(|c|c.to_ascii_lowercase())==Some('d'){let old=chars.pop().unwrap();chars.push(if old.is_uppercase(){'Đ'}else{'đ'});prev=x;continue}
 if (mode==Mode::Telex||mode==Mode::Simple)&&x=='['{chars.push('ư');prev=x;continue}
 if (mode==Mode::Telex||mode==Mode::Simple)&&x==']'{chars.push('ơ');prev=x;continue}
 if mode==Mode::Telex&&x=='w'&&!chars.iter().any(|c|vowel(*c).is_some()){chars.push(if c.is_uppercase(){'Ư'}else{'ư'});prev=x;continue}
 // Repeated tone key cancels tone and preserves one literal key (English escape).
 if x==prev&&tone!=0&&(mode==Mode::Telex||mode==Mode::Simple)&&matches!(x,'s'|'f'|'r'|'x'|'j'){tone=0;chars.push(c);prev=x;continue}
 chars.push(c);prev=x;
 }
 if tone>0{let all:Vec<usize>=chars.iter().enumerate().filter_map(|(i,c)|vowel(*c).map(|_|i)).collect();
 if let Some(&last)=all.last(){let mut at=last;
 if let Some(&p)=all.iter().find(|&&p|matches!(vowel(chars[p]),Some((4|7|8,_)))){at=p}
 else if all.len()>1&&last+1<chars.len(){at=all[0]}
 let(g,_)=vowel(chars[at]).unwrap();chars[at]=make(chars[at],g,tone);}}
 chars.iter().collect()
}
fn case(raw:&str,s:&str)->String{if raw.chars().all(|c|c.is_uppercase()){s.to_uppercase()}else if raw.chars().next().unwrap().is_uppercase(){let mut c=s.chars();format!("{}{}",c.next().unwrap().to_uppercase(),c.collect::<String>())}else{s.into()}}
pub fn convert(input:&str,mode:Mode)->String{let mut out=String::new();let mut word=String::new();for c in input.chars(){if c.is_ascii_alphanumeric()||"[]^+('?~.".contains(c)&&mode==Mode::Viqr || "[]".contains(c)&&(mode==Mode::Telex||mode==Mode::Simple){if word.len()==MAX_RAW{out.push_str(&compose(&word,mode));word.clear()}word.push(c)}else{out.push_str(&compose(&word,mode));word.clear();out.push(c)}}out.push_str(&compose(&word,mode));out}
#[cfg(feature="wasm")]mod wasm{use super::*;use wasm_bindgen::prelude::*;#[wasm_bindgen]pub fn type_text(s:&str,mode:&str)->String{convert(s,Mode::from(mode))}}
pub struct Session {raw:String,display:String,mode:Mode,out:CString}
#[repr(C)]pub struct Action{pub kind:u32,pub delete_utf16:u32,pub cursor_utf16:u32,pub pass_key:bool,pub text:*const c_char}
#[unsafe(no_mangle)]pub extern "C" fn gotivi_new(mode:u32)->*mut Session{let m=match mode{1=>Mode::Simple,2=>Mode::Vni,3=>Mode::Viqr,4=>Mode::English,_=>Mode::Telex};Box::into_raw(Box::new(Session{raw:String::new(),display:String::new(),mode:m,out:CString::new("").unwrap()}))}
#[unsafe(no_mangle)]pub unsafe extern "C" fn gotivi_free(p:*mut Session){if !p.is_null(){drop(Box::from_raw(p));}}
#[unsafe(no_mangle)]pub unsafe extern "C" fn gotivi_feed(p:*mut Session,key:*const c_char)->Action{let pass=Action{kind:0,delete_utf16:0,cursor_utf16:0,pass_key:true,text:std::ptr::null()};if p.is_null()||key.is_null(){return pass}let s=&mut*p;let Ok(k)=CStr::from_ptr(key).to_str() else{return pass};if k=="Backspace"{s.raw.pop();}else if k.chars().count()==1&&k.chars().all(|c|c.is_ascii_alphanumeric()||"[]^+('?~.".contains(c)){if s.raw.len()>=MAX_RAW{s.raw.clear();s.display.clear();return pass}s.raw.push_str(k);}else{s.raw.clear();s.display.clear();return pass}let new=compose(&s.raw,s.mode);let old=std::mem::replace(&mut s.display,new.clone());s.out=CString::new(new).unwrap();Action{kind:1,delete_utf16:old.encode_utf16().count() as u32,cursor_utf16:s.display.encode_utf16().count() as u32,pass_key:false,text:s.out.as_ptr()}}
#[cfg(test)]mod test{use super::*;#[test]fn telex(){assert_eq!(convert("Tieengs Vieejt",Mode::Telex),"Tiếng Việt");assert_eq!(compose("dd",Mode::Telex),"đ");assert_eq!(compose("thuowng",Mode::Telex),"thương");}#[test]fn macro_test(){assert_eq!(convert("ko vn",Mode::Telex),"không Việt Nam");}}
