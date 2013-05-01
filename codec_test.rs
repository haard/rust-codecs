
use core::unicode;

mod codec_base;



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


pub trait Encodeable {
    pub fn encode(&self, encoding: &str) -> ~[u8];
}

impl Encodeable for ~str {
    pub fn encode(&self, encoding: &str) -> ~[u8] {
        return codec_base::encode(*self, encoding);
    }
}

pub trait Decodeable {
    pub fn decode(&self, encoding: &str) -> ~str;
}

impl Decodeable for ~[u8] {
    pub fn decode(&self, encoding: &str) -> ~str {
        return codec_base::decode(*self, encoding);
    }
}


fn main() {

        let file_contents = get_file_contents_u8("test.binary");
        let hej : ~str = ~"hej";
        hej.encode("8859-15");
//        io::println(file_contents.decode(~"huh"))  ;
//        io::println(fmt!("%?", file_contents.decode(~"huh").encode(~"")));

}


