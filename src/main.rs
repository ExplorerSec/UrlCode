use std::env;
mod url_basic;

use url_basic::{UrlCodec, EncodingType};


fn show_help(){
    print!("\u{001b}[1;42mUrlCode 0.0.4, By ExpZero\u{001b}[0m\n\n"); // 使用颜色转义
    println!("Usage: UrlCode [Command] [Url String] [Optional]");
    println!("Command:");
    println!("    e|-e|encode|-encode|--encode        encode the url");
    println!("    d|-d|decode|-decode|--decode        decode the url");
    println!("Url String:");
    println!("    the string to encode or decode.");
    println!("Optional:");
    println!("    -utf8|--utf8                        codec with utf8 [default]");
    println!("    -utf16[le]|--utf16[le]              codec with UTF-16 [little-endian]");
    println!("    -utf16be|--utf16be                  codec with UTF-16 big-endian");
}
fn main() {
    let args:Vec<String> = env::args().collect();
    let urlcodec =UrlCodec::new();
    let utf:EncodingType;
    match args.len(){
        3 => utf = EncodingType::UTF8,
        4 => match &args[3][..] {
            "-utf8"|"--utf8" => utf = EncodingType::UTF8,
            "-utf16be"|"--utf16be" => utf = EncodingType::UTF16BE,
            "-utf16"|"--utf16"|"-utf16le"|"--utf16le" => utf = EncodingType::UTF16LE,
            _ => {
                utf = EncodingType::UTF8;
                println!("[Warning] Invilad EncodingType, Using UTF8")
            },
        } 
        _ => {
            show_help();
            return;
        }
    }
    match &args[1][..] {
        "e"|"-e"|"encode"|"-encode"|"--encode" => {
            println!("{}",urlcodec.url_encode(&args[2],utf).unwrap());
        },
        "d"|"-d"|"decode"|"-decode"|"--decode" => {
            match urlcodec.url_decode(&args[2]) {
                Ok(s) => println!("{s}"),
                _ => println!("[Error] Invalid Char Found"),
            }
        }
        _ => show_help(),
    }
}

