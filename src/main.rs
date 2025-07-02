use std::{fs, io::{BufRead, BufReader, Write}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("listening at :7878");
    for incoming in listener.incoming() {
        let mut stream = match incoming {
            Ok(v) => v,
            Err(_) => continue
        };

        let req : Vec<String> = {
            let buf = BufReader::new(&stream);
            buf
                .lines()
                .map(|f|f.unwrap())
                .take_while(|b| !b.is_empty())
                .collect()
        };

        let body = String::from_utf8(fs::read("./views/index.html").unwrap()).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{body}", body.len()); 

        stream.write_all(response.as_bytes()).unwrap();
    }
}
