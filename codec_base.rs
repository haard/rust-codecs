use core::hashmap::HashMap;
mod iso_8859_15;

pub fn encode(string:&str, encoding: &str) -> ~[u8] {
    let charmap = match (encoding) {
        &"8859-15" => { reverse_charmap(iso_8859_15::charmap()) }
        _ => {fail!("Unmapped!")}
    };
    return encode_charmap(string, charmap);
}

pub fn decode(string: &[u8], encoding: &str) -> ~str {
    let charmap = match (encoding) {
        &"8859-15" => { iso_8859_15::charmap() }
        _ => {fail!("Unmapped!")}
    };
    return decode_charmap(string, charmap);
}

fn encode_charmap(string: &str, reverse_charmap: ~HashMap<char, u8>) -> ~[u8] {
    let mut out = ~[];
    for str::each_char(string) |chr| {
        out.push(*reverse_charmap.get(&chr));
    }
    return out; 
}

fn decode_charmap(data : &[u8], charmap :&[~str]) -> ~str {
    let mut out = ~"";  
    for data.each |chr| {
        out += charmap[*chr];
    }
    return out;
}

fn reverse_charmap(charmap : &[~str])-> ~HashMap<char, u8> {
    let mut reverse = ~HashMap::new();
    let mut n = 0;
    while n < charmap.len() {
       reverse.insert(str::char_at(charmap[n], 0), n as u8);
       n += 1;
   }
   return reverse;
}


pub trait Encodeable  {
    pub fn encode(&self, encoding: &str) -> ~[u8];
}

impl Encodeable for ~str {
    pub fn encode(&self, encoding: &str) -> ~[u8] {
        return encode(*self, encoding);
    }
}

pub trait Decodeable {
    pub fn decode(&self, encoding: &str) -> ~str;
}

impl Decodeable for ~[u8] {
    pub fn decode(&self, encoding: &str) -> ~str {
        return decode(*self, encoding);
    }
}
