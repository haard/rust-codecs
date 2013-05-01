
use core::unicode;
use codec_base::Encodeable;
use codec_base::Decodeable;

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




fn main() {

        let file_contents = get_file_contents_u8("test.binary");
        let hej : ~str = ~"hej";
        hej.encode("8859-15");
//        io::println(file_contents.decode(~"huh"))  ;
//        io::println(fmt!("%?", file_contents.decode(~"huh").encode(~"")));

}


