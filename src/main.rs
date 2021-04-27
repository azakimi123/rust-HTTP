fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..];
    // //short cut to create a string slice
    // let string_borrow: &str = &string[10..];
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    //double colon is to access associtated functions
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self{
        Self {
            addr: addr
        }
    }
    //methods have a first parameter "self" eg. "this" in other languages
    // self points to the instance of the struct
    fn run(self){
        println!("Server is listening on {}", self.addr)
    }
}