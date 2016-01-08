use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    
    match arguments.len() {
        1 => { loop { println!("y") } },
        _ => { loop { println!("{}", arguments[1]) } },
    }
}
