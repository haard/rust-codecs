
use core::unicode;
use codecs::Encodeable;
use codecs::Decodeable;
mod codecs;



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


fn main() {
        let file_contents = get_file_contents_u8("test.binary");
        io::println(file_contents.decode(~"iso_8859_15"))  ;
        io::println(fmt!("%?", file_contents.decode(~"iso_8859_15").encode(~"iso_8859_15")));
}


