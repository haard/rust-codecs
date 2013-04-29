
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
    let mut mapping = ~[];

    for str::each_line(contents) |line| {
        if line.len() == 0 || line[0] == '#' as u8 {
            loop; //ignore
        }
        let mut v = ~[];
        for str::each_split_char_nonempty(line, '\t') |word| {
            if word == "#" {break;}
            v.push(word);
        };
        mapping.push(v)
    }

    let mut array = ~[];

    array.push(~"pub fn charmap() -> [&'static str, .. 256]{ return [");

    for mapping.each() |m| {
        let t = int::from_str_radix(str::slice(m[0],2,m[0].len()), 16);
        let v = (int::from_str_radix(str::slice(m[1],2, 4), 16),int::from_str_radix(str::slice(m[1],4,6), 16));
        match (t, v) {

            (Some(tvi), (Some(vv1), Some(vv2))) => {
                let tv = tvi as u8;
                let vv = @[vv1 as u8, vv2 as u8];
                let tvhex = int::to_str_radix(tvi, 16);
                let vvhex = int::to_str_radix(vv1, 16) + int::to_str_radix(vv2, 16);

                if str::is_utf8(vv) {
                    if(vv[0] > 0) {
                        let thestr = str::escape_unicode(str::from_bytes(vv));
                        array.push(fmt!("\"\\u%s%s\", // 0x%s", str::slice(thestr,2,4), str::slice(thestr,6,8), tvhex));
                    } else {
                        let mut astr = str::from_byte(vv2 as u8);
                        if unicode::general_category::Cc(str::char_at(astr, 0)) {
                            astr = str::escape_default(astr);
                        }
                        array.push(fmt!("\"%s\", // 0x%s", astr, tvhex));
                    }
                } else {
                    if(vv[0] > 0) {
                        array.push(fmt!("\"\\x%s\", // 0x%s", vvhex, tvhex));
                    } else {
                        array.push(fmt!("\"\\x%s\", // 0x%s", str::slice(vvhex, 1, vvhex.len()), tvhex));
                    }
                }
            },
            (_, _) => {io::println(fmt!("Error: %? %?", m[0], m[1]));}
        }
        
    }
    array.push(~"];}");
    
    return array;
}


fn main() {
    if os::args().any(|x| *x == ~"generate") {
        let charmap = get_charmap();
        emit(charmap);
    } else {
    
        let charmap = iso_8859_15::charmap();
        let reverse = reverse_charmap(charmap);
        let file = get_file_contents_u8("test.binary");
        io::println(fmt!("File: %?", file));
        for file.each |chr| {
            io::println(charmap[*chr]);
            assert!(chr == reverse.get(&charmap[*chr]))
        }
    }

}

fn reverse_charmap(charmap :&[&'static str])-> ~LinearMap<&'static str, u8> {
    let mut reverse = ~core::hashmap::linear::LinearMap::new();
    let mut n = 0;
    while n < charmap.len() {
       reverse.insert(charmap[n], n as u8);
       n += 1;
   }
   return reverse;
}

