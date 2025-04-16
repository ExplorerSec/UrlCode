use std::env;
mod url_basic;

use url_basic::url_encode;
use url_basic::url_decode;


fn show_help(){
    print!("\u{001b}[1;42mUrlCode 0.0.1, By ExpZero\u{001b}[0m\n\n"); // 使用颜色转义
    println!("Usage: UrlCode [Command] [Url String]");
    println!("Command:");
    println!("    e|-e|encode|-encode|--encode        encode the url");
    println!("    d|-d|decode|-decode|--decode        decode the url");
    println!("Url String:");
    println!("    the string to encode or decode.");
}
fn main() {
    let args:Vec<String> = env::args().collect();
    match args.len(){
        1|2 => show_help(),
        _ =>{
            match &(args[1])[..] {
                "e"|"-e"|"encode"|"-encode"|"--encode" => println!("{}",url_encode(&args[2])), 
                "d"|"-d"|"decode"|"-decode"|"--decode" => println!("{}",url_decode(&args[2])),
                _ => show_help(),
            }
        }
    } 
    
}
