use gotivi::{convert,Mode}; 
fn main() {
    let mut args=std::env::args().skip(1);
    let mut mode=Mode::Telex;let mut text=None;let mut demo=false;
    while let Some(arg)=args.next() {
        match arg.as_str(){
            "--mode"=>mode=Mode::parse(&args.next().unwrap_or_default()),
            "--text"=>text=args.next(),
            "--demo"=>demo=true,
            _=>{eprintln!("Usage: gotivi [--demo] [--mode telex|simple|vni|viqr|english] [--text TEXT]");return}
        }
    }
    if demo {
        for s in ["Tieengs Vieejt","thuowng","dd","mixx","hoaf","ko vn"] {
            println!("{s} => {}",convert(s,mode));
        }
    } else if let Some(s)=text {println!("{}",convert(&s,mode))}
    else {use std::io::{self,BufRead};for line in io::stdin().lock().lines().map_while(Result::ok){println!("{}",convert(&line,mode))}}
}
