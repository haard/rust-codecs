use std::hashmap::HashMap;
mod iso_8859_1;
mod iso_8859_10;
mod iso_8859_11;
mod iso_8859_13;
mod iso_8859_14;
mod iso_8859_15;
mod iso_8859_16;
mod iso_8859_2;
mod iso_8859_3;
mod iso_8859_4;
mod iso_8859_5;
mod iso_8859_6;
mod iso_8859_7;
mod iso_8859_8;
mod iso_8859_9;
pub fn encode(string:&str, encoding: &str) -> ~[u8] {
        let charmap = match (encoding) {        &"iso_8859_1" => { reverse_charmap(iso_8859_1::charmap()) }
        &"iso_8859_10" => { reverse_charmap(iso_8859_10::charmap()) }
        &"iso_8859_11" => { reverse_charmap(iso_8859_11::charmap()) }
        &"iso_8859_13" => { reverse_charmap(iso_8859_13::charmap()) }
        &"iso_8859_14" => { reverse_charmap(iso_8859_14::charmap()) }
        &"iso_8859_15" => { reverse_charmap(iso_8859_15::charmap()) }
        &"iso_8859_16" => { reverse_charmap(iso_8859_16::charmap()) }
        &"iso_8859_2" => { reverse_charmap(iso_8859_2::charmap()) }
        &"iso_8859_3" => { reverse_charmap(iso_8859_3::charmap()) }
        &"iso_8859_4" => { reverse_charmap(iso_8859_4::charmap()) }
        &"iso_8859_5" => { reverse_charmap(iso_8859_5::charmap()) }
        &"iso_8859_6" => { reverse_charmap(iso_8859_6::charmap()) }
        &"iso_8859_7" => { reverse_charmap(iso_8859_7::charmap()) }
        &"iso_8859_8" => { reverse_charmap(iso_8859_8::charmap()) }
        &"iso_8859_9" => { reverse_charmap(iso_8859_9::charmap()) }
    _ => {fail!(fmt!("Unmapped: '%?'!", encoding))}
        };
        return encode_charmap(string, charmap);
    }
pub fn decode(string: &[u8], encoding: &str) -> ~str {
        let charmap = match (encoding) {
        &"iso_8859_1" => { iso_8859_1::charmap() }        &"iso_8859_10" => { iso_8859_10::charmap() }        &"iso_8859_11" => { iso_8859_11::charmap() }        &"iso_8859_13" => { iso_8859_13::charmap() }        &"iso_8859_14" => { iso_8859_14::charmap() }        &"iso_8859_15" => { iso_8859_15::charmap() }        &"iso_8859_16" => { iso_8859_16::charmap() }        &"iso_8859_2" => { iso_8859_2::charmap() }        &"iso_8859_3" => { iso_8859_3::charmap() }        &"iso_8859_4" => { iso_8859_4::charmap() }        &"iso_8859_5" => { iso_8859_5::charmap() }        &"iso_8859_6" => { iso_8859_6::charmap() }        &"iso_8859_7" => { iso_8859_7::charmap() }        &"iso_8859_8" => { iso_8859_8::charmap() }        &"iso_8859_9" => { iso_8859_9::charmap() } _ => {fail!(fmt!("Unmapped: '%?'!", encoding))}
        };
        return decode_charmap(string, charmap);}


fn encode_charmap(string: &str, reverse_charmap: ~HashMap<char, u8>) -> ~[u8] {
    let mut out = ~[];
    for string.iter().advance |chr| {
        out.push(*reverse_charmap.get(&chr));
    }
    return out; 
}

fn decode_charmap(data : &[u8], charmap :[&'static str, .. 256]) -> ~str {
    let mut out = ~"";  
    for data.iter().advance |chr| {
        out = out + charmap[*chr];
    }
    return out;
}

fn reverse_charmap(charmap : [&'static str, .. 256])-> ~HashMap<char, u8> {
    let mut reverse = ~HashMap::new();
    let mut n = 0;
    while n < charmap.len() {
       reverse.insert(charmap[n].char_at(0), n as u8);
       n += 1;
   }
   return reverse;
}

pub trait Encodeable  {
    fn encode(&self, encoding: &str) -> ~[u8];
}

impl Encodeable for ~str {
    pub fn encode(&self, encoding: &str) -> ~[u8] {
        return encode(*self, encoding);
    }
}

pub trait Decodeable {
    fn decode(&self, encoding: &str) -> ~str;
}

impl Decodeable for ~[u8] {
    pub fn decode(&self, encoding: &str) -> ~str {
        return decode(*self, encoding);
    }
}
