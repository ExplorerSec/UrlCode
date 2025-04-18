use std::collections::{HashMap, HashSet};

pub struct UrlCodec {
    valid_char_set:HashSet<char>,
    _escape_c2d_map:HashMap<&'static str,&'static str>,
    _escape_d2c_map:HashMap<&'static str,&'static str>,
}

impl UrlCodec {
    pub fn new() -> UrlCodec{
        UrlCodec{
            valid_char_set:HashSet::from([
                'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                '0','1','2','3','4','5','6','7','8','9','-','_','.','~']),
            _escape_c2d_map:HashMap::from([
                (" ", "%20"),("!", "%21"),("\"", "%22"),("'", "%27"),("(", "%28"),(")", "%29"),
                ("*", "%2A"),("+", "%2B"),(",", "%2C"),("/", "%2F"),(":", "%3A"),(";", "%3B"),
                ("<", "%3C"),(">", "%3E"),("@", "%40"),("[", "%5B"),("]", "%5D"),("\\", "%5C"),
                ("_", "%5F"),("^", "%5E"),("`", "%60"),("{", "%7B"),("|", "%7C"),("}", "%7D"),//("~", "%7E"),
                ]),
            _escape_d2c_map:HashMap::from([
                ("%20"," "),("%21","!"),("%22","\""),("%27","'"),("%28","("),("%29",")"),
                ("%2A","*"),("%2B","+"),("%2C",","),("%2F","/"), ("%3A",":"),("%3B",";"), ("%3C","<"),
                ("%3E",">"), ("%40","@"),("%5B","["), ("%5D","]"),("%5C","\\"),("%5F","_"),
                ("%5E","^"), ("%60","`"),("%7B","{"), ("%7C","|"),("%7D","}"), ("%7E","~"),
                ])
        }
    }
    
    pub fn url_decode(&self,s:&str)->Result<String,()>{
        self.url_utf8_decode(s)
    }
    
    pub fn url_encode(&self, s:&str)->Result<String,()>{
        self.url_utf16be_encode(s)
    }

    pub fn _url_basic_encode(&self, s:&str)->String{
        let mut out = String::from(s);
        
        for i in &self._escape_c2d_map{
            out = out.replace(i.0, i.1);
        }
        out
    
    }
    
    pub fn _url_basic_decode(&self, s:&str)->String{
        let mut out = String::from(s);
        for i in &self._escape_d2c_map{
            out = out.replace(i.0, i.1);
        }
        out
    }
    
    fn url_utf16be_encode(&self, s:&str)->Result<String,()>{
        let mut out:String = String::with_capacity(s.len());
        for ch in s.chars(){
            if self.valid_char_set.contains(&ch){
                out.push(ch);
            }else {
                let hex_list = (ch as u32).to_be_bytes();
                for hex in hex_list{
                    if hex != 0{
                        let tmp_str =format!("%{:02X}",hex);
                            out.push_str(&tmp_str);
                    }
                }
            }
        }    
            
        Ok(out)       
    } 
        
    fn url_utf8_decode(&self, s:&str)->Result<String,()>{       
            
            // 将整串以 '%' 为分隔符，划分为多个字串
            let v: Vec<&str> = s.split('%').collect();
            // 最前面的字串没有 '%'，直接作为新的串；并预留合适大小空间 
            let mut out:String = String::from(v[0]); 
            out.reserve(s.len());
            
            let mut vec: Vec<u8> = Vec::new();
            for &s_sub in v.iter().skip(1){ 
                // first 是百分号修饰的两个十六进制数, last 是剩下的字符串
                let (first, last) = s_sub.split_at(2); 
                match u8::from_str_radix(first, 16){
                    Ok(byte) => vec.push(byte),
                    _ => return  Err(()),
                }
                if !last.is_empty(){
                    if !vec.is_empty(){
                        match String::from_utf8(vec){
                            Ok(s) => out.push_str(&s),
                            _ => return Err(()),
                        }
                        vec=Vec::new();
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
        
}
