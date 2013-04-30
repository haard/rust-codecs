
use core::unicode;
use core::hashmap::linear::LinearMap;
mod iso_8859_15;

fn get_file_contents(path: &str) -> ~str {
    let read_result = io::read_whole_file_str(&path::Path(path));
    match read_result {
        Err(msg) => {
            fail!(fmt!("Error: %?", msg)); 
        },
        Ok(contents) => {
            return contents;
        }
    }
}

fn get_file_contents_u8(path: &str) -> ~[u8] {
    let read_result = io::read_whole_file(&path::Path(path));
    match read_result {
        Err(msg) => {
            fail!(fmt!("Error: %?", msg)); 
        },
        Ok(contents) => {
            return contents;
        }
    }
}

fn emit(lines: &[~str]) {
    for lines.each() |line| {
        io::println(*line);        
    }
}

fn get_charmap() -> ~[~str]{

    let contents: ~str = get_file_contents("8859-15.txt");
    let mut mappings = ~[]; // holder for lines from unicode.org specs

    for str::each_line(contents) |line| {
        if line.len() == 0 || line[0] == '#' as u8 {
            loop; //ignore
        }
        let mut v = ~[]; // tokenized line
        for str::each_split_char_nonempty(line, '\t') |word| {
            if word == "#" {break;}
            v.push(word);
        };
        mappings.push(v)
    }

    let mut array = ~[]; //Rust source output

    array.push(~"pub fn charmap() -> [&'static str, .. 256]{ return [");

    for mappings.each() |m| {
        let code = int::from_str_radix(str::slice(m[0],2,m[0].len()), 16);
        let ucode = (int::from_str_radix(str::slice(m[1],2, 4), 16),int::from_str_radix(str::slice(m[1],4,6), 16));
        match (code, ucode) {

            (Some(code), (Some(ucode_hi), Some(ucode_lo))) => {
                let ucode_vec    = @[ucode_hi as u8, ucode_lo as u8];
                let tvhex = int::to_str_radix(code, 16);
                let ucode_vechex = int::to_str_radix(ucode_hi, 16) + int::to_str_radix(ucode_lo, 16);

                if str::is_utf8(ucode_vec) { // Legal unicode
                    if(ucode_vec[0] > 0) { // Two-byte
                        let thestr = str::escape_unicode(str::from_bytes(ucode_vec));
                        array.push(fmt!("\"\\u%s%s\", // 0x%s", str::slice(thestr,2,4), str::slice(thestr,6,8), tvhex));
                    } else { // Single-byte
                        let mut astr = str::from_byte(ucode_lo as u8);
                        if unicode::general_category::Cc(str::char_at(astr, 0)) { //control chars
                            astr = str::escape_default(astr);
                        } else if ['"', '\\'].any(|x| *x==str::char_at(astr, 0)) {
                            io::println("Escapinating   "); 
                            astr = str::escape_default(astr);
                        }
                        array.push(fmt!("\"%s\", // 0x%s", astr, tvhex));
                    }
                } else {
                    if(ucode_vec[0] > 0) {
                        array.push(fmt!("\"\\x%s\", // 0x%s", ucode_vechex, tvhex));
                    } else {
                        array.push(fmt!("\"\\x%s\", // 0x%s", str::slice(ucode_vechex, 1, ucode_vechex.len()), tvhex));
                    }
                }
            },
            (_, _) => {io::println(fmt!("Error: %? %?", m[0], m[1]));}
        }
        
    }
    array.push(~"];}");
    
    return array;
}

fn decode(data : &[u8], charmap :&[~str]) -> ~str {
    let mut out = ~"";  
    for data.each |chr| {
        out += charmap[*chr];
    }
    return out;
}
    
fn encode(string: &str, reverse_charmap: ~LinearMap<char, u8>) -> ~[u8] {
    let mut out = ~[];
    for str::each_char(string) |chr| {
        out.push(*reverse_charmap.get(&chr));
    }
    return out; 
}

fn main() {
    if os::args().any(|x| *x == ~"generate") {
        let charmap = get_charmap();
        emit(charmap);
    } else {
    
        let charmap = iso_8859_15::charmap();
        let reverse = reverse_charmap(charmap);
        let file = get_file_contents_u8("test.binary");

        io::println(decode(file, charmap))  ;
        io::println(fmt!("%?", encode(decode(file, charmap), reverse)));
    }

}

fn reverse_charmap(charmap : &[~str])-> ~LinearMap<char, u8> {
    let mut reverse = ~core::hashmap::linear::LinearMap::new();
    let mut n = 0;
    while n < charmap.len() {
       reverse.insert(str::char_at(charmap[n], 0), n as u8);
       n += 1;
   }
   return reverse;
}

