use std::collections::HashMap;

pub fn url_basic_encode(s:&str)->String{
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

pub fn url_basic_decode(s:&str)->String{
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


pub fn url_utf8_decode(s:&str)->Result<String,()>{
    // 返回错误：不是合法的UTF8编码文字
    
    let v: Vec<&str> = s.split('%').collect();
    
    let mut out:String = String::from(v[0]); 
    out.reserve(s.len());
    
    let mut vec: Vec<u8> = Vec::new();
    for i in 1..v.len(){
        // first 是百分号修饰的两个十六进制数,
        // last 是剩下的字符串
        let (first, last) = v[i].split_at(2); 
         
        if let Ok(byte) = u8::from_str_radix(first, 16){
            vec.push(byte);
        }
        else {
            return Err(());
        }
            
        if !last.is_empty(){
            if !vec.is_empty(){
                match String::from_utf8(vec.clone()){
                    Ok(s) => out.push_str(&s),
                    _ => return Err(()),
                }
                vec.clear();
            }
            out.push_str(last);
        }
    }
    if !vec.is_empty(){
        match String::from_utf8(vec){
            Ok(s) => out.push_str(&s),
            _ => return Err(()),
        }
    }

    Ok(out)
}


pub fn url_decode(s:&str)->Result<String,()>{
    match url_utf8_decode(&url_basic_decode(s)) {
        Ok(s) => Ok(s),
        Err(_) => Err(()),
    }       
}