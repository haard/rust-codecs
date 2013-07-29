

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
