use std::{env, collections::HashMap};

fn url_encode(s:&str)->String{
    let map: HashMap<&str, &str> = HashMap::from([
        (" ", "%20"),
        ("!", "%21"),
        ("\"", "%22"),
        ("'", "%27"),
        ("(", "%28"),
        (")", "%29"),
        ("*", "%2A"),
        ("+", "%2B"),
        (",", "%2C"),
        ("/", "%2F"),
        (":", "%3A"),
        (";", "%3B"),
        ("<", "%3C"),
        (">", "%3E"),
        ("@", "%40"),
        ("[", "%5B"),
        ("]", "%5D"),
        ("\\", "%5C"),
        ("_", "%5F"),
        ("^", "%5E"),
        ("`", "%60"),
        ("{", "%7B"),
        ("|", "%7C"),
        ("}", "%7D"),
        //("~", "%7E"),
        ]);
        let mut out = String::from(s);
        for i in map{
            out = out.replace(i.0, i.1);
        }
        out

}

fn url_decode(s:&str)->String{
    let map: HashMap<&str, &str> = HashMap::from([
        ("%20"," "),
        ("%21","!"),
        ("%22","\""),
        ("%27","'"),
        ("%28","("),
        ("%29",")"),
        ("%2A","*"),
        ("%2B","+"),
        ("%2C",","),
        ("%2F","/"),
        ("%3A",":"),
        ("%3B",";"),
        ("%3C","<"),
        ("%3E",">"),
        ("%40","@"),
        ("%5B","["),
        ("%5D","]"),
        ("%5C","\\"),
        ("%5F","_"),
        ("%5E","^"),
        ("%60","`"),
        ("%7B","{"),
        ("%7C","|"),
        ("%7D","}"),
        ("%7E","~"),
        ]);
        let mut out = String::from(s);
        for i in map{
            out = out.replace(i.0, i.1);
        }
        out
    }

fn show_help(){
    println!("\u{001b}[1;42mUrlCode 0.0.1, By ExpZero\u{001b}[0m"); // 使用颜色转义
    println!("");
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
