use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use crate::one_thread::thread_pool::ThreadPool;

mod thread_pool;

pub fn single_thread_web() {
    tcp_listen();
}

fn tcp_listen() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap_or_else(|e| panic!("bind failed! {}", e.to_string()));

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        if let Ok(tcp_stream) = stream {
            // tcp_stream 超出作用域后
            println!("connection establish !");
            thread_pool.execute(|| {
                handle_connection(tcp_stream);
            });
        } else {
            println!("connection failed !");
        }
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut tcp_stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("request: {:#?}", http_request);
    let (response_status, filename) = if http_request[0] == "GET /index.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let response_contents = fs::read_to_string(format!("./src/site/{filename}")).unwrap();
    let response_length = response_contents.len();
    let http_response =
        format!("{response_status}\r\nContent-Length: {response_length}\r\n\r\n{response_contents}");

    tcp_stream.write(http_response.as_bytes()).unwrap_or_else(|e| panic!("response failed! {}", e.to_string()));
}
// impl Drop for TcpStream {
//     fn drop(&mut self) {
//         todo!()
//     }
// }