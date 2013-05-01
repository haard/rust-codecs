use core::unicode;
//use io::WriterUtil;

fn main() {
    let args = os::args();
    let mut encodings = ~[];
    args.slice(1, args.len()).each(|x| {
        let charmap = get_charmap(x);
        emit(x, charmap);
        encodings.push(fmt!("%s", *x));
        true
    });

    let mut codecs_src = ~[];
    encodings.each(|x| {
        let line = fmt!("mod %s;", *x);
        codecs_src.push(line);
        true
    });

    let head = "fn encode(string:&str, encoding: &str) {
        match (encoding) {

        }
    ";

    emit(&~"codecs", codecs_src);
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

fn emit(name: &~str, lines: &[~str]) {
    let maybe_writer = io::file_writer(&path::Path(name + ~".rs"), &[io::Create]);
    match maybe_writer {
        Err(msg) => {
            fail!(fmt!("Error: %?", msg)); 
        },
        Ok(writer) => {
            for lines.each() |line| {
                writer.write_str(*line);        
            }
        }
    }
}

fn get_charmap(name: &~str) -> ~[~str] {
    let contents: ~str = get_file_contents(fmt!("%s.txt", *name));
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