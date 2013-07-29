use std::{unicode,io,os,path,str,int};

fn main() {
    let args = os::args();


    let mut encodings = ~[];
    for args.slice(1, os::args().len()).iter().advance |filename| {
        io::println(fmt!("Reading %s", *filename));
        let name = str::replace(filename.slice(0, filename.len()-4), &"-", &"_");
        let charmap = get_charmap(filename.slice(0, filename.len()));
        emit(name, charmap);
        encodings.push(fmt!("%s", name));
    }

    let mut codecs_src = ~[];
    let mut encode_src = ~[];
    let mut decode_src = ~[];
    
    codecs_src.push(~"use std::hashmap::HashMap;\n");
    for encodings.iter().advance |x| {
        codecs_src.push(fmt!("mod %s;\n", *x));
        encode_src.push(fmt!("&\"%s\" => { reverse_charmap(%s::charmap()) }\n", *x, *x));
        decode_src.push(fmt!("&\"%s\" => { %s::charmap() }", *x, *x));
    }
    
    codecs_src.push(~"pub fn encode(string:&str, encoding: &str) -> ~[u8] {
        let charmap = match (encoding) {");
    codecs_src.push_all_move(encode_src);
    codecs_src.push(~"    _ => {fail!(fmt!(\"Unmapped: '%?'!\", encoding))}
        };
        return encode_charmap(string, charmap);
    }\n");

    codecs_src.push(~"pub fn decode(string: &[u8], encoding: &str) -> ~str {
        let charmap = match (encoding) {\n");
    codecs_src.push_all_move(decode_src);
    codecs_src.push(~" _ => {fail!(fmt!(\"Unmapped: '%?'!\", encoding))}
        };
        return decode_charmap(string, charmap);}\n");

    codecs_src.push(get_file_contents("codec_base.rs"));

    emit(&"codecs", codecs_src);
}

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

fn emit(name: &str, lines: &[~str]) {
    let maybe_writer = io::file_writer(&path::Path(fmt!("%s.rs", name)), &[io::Create]);
    match maybe_writer {
        Err(msg) => {
            fail!(fmt!("Error: %?", msg)); 
        },
        Ok(writer) => {
            for lines.iter().advance |line| {
                writer.write_str(*line);        
            }
        }
    }
}

fn get_charmap(name: &str) -> ~[~str] {
    let contents: ~str = get_file_contents(name);
    let mut mappings = ~[]; // holder for lines from unicode.org specs

    for contents.line_iter().advance |line| {
        if line.len() == 0 || line[0] == '#' as u8 {
            loop; //ignore
        }
        let mut v = ~[]; // tokenized line
        for line.split_str_iter("\t").advance |word| {
            if word == "#" {break;}
            v.push(word);
        };
        mappings.push(v)
    }

    let mut array = ~[]; //Rust source output

    array.push(fmt!("pub fn charmap() -> [&'static str, .. %?]{ return [", mappings.len()));

    for mappings.iter().advance |m| {
        if m[1].len() < 6 {
            array.push(fmt!("\"\\uFFFD\", // 0x%s unmapped\n", m[0]));
        } else {
        let code = int::from_str_radix(m[0].slice(2,m[0].len()), 16);
        let ucode = (int::from_str_radix(m[1].slice(2, 4), 16),int::from_str_radix(m[1].slice(4,6), 16));
        match (code, ucode) {

            (Some(code), (Some(ucode_hi), Some(ucode_lo))) => {
                let ucode_vec    = @[ucode_hi as u8, ucode_lo as u8];
                let tvhex = int::to_str_radix(code, 16);
                let ucode_vechex = int::to_str_radix(ucode_hi, 16) + int::to_str_radix(ucode_lo, 16);

                if str::is_utf8(ucode_vec) { // Legal unicode
                    if(ucode_vec[0] > 0) { // Two-byte
                        let thestr = str::from_bytes(ucode_vec).escape_unicode();
                        array.push(fmt!("\"\\u%s%s\", // 0x%s\n", thestr.slice(2,4), thestr.slice(6,8), tvhex));
                    } else { // Single-byte
                        let mut astr = str::from_byte(ucode_lo as u8);
                        if unicode::general_category::Cc(astr.char_at(0)) { //control chars
                            astr = astr.escape_default();
                        } else if '"' ==astr.char_at(0) || '\\' == astr.char_at(0) {
                            astr = astr.escape_default();
                        }
                        array.push(fmt!("\"%s\", // 0x%s\n", astr, tvhex));
                    }
                } else {
                    if(ucode_vec[0] > 0) {
                        array.push(fmt!("\"\\x%s\", // 0x%s\n", ucode_vechex, tvhex));
                    } else {
                        array.push(fmt!("\"\\x%s\", // 0x%s\n", ucode_vechex.slice(1, ucode_vechex.len()), tvhex));
                    }
                }
            },
            (_, _) => {

                io::println(fmt!("Unmapped: %?", m[0]));
                array.push(fmt!("\"\\uFFFD\", // 0x%s unmapped\n", m[0]))
                
            }
        }
    }
        
    }
    array.push(~"];}");
    
    return array;
}