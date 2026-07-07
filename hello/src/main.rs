use hello::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // port 7878 is not usual for HTTP, 7878 = rust
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        // check each stream for errors, crash on error. Unwrap is ok here
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // grab a buffer reader for streams
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    // normally you would add error handling instead of unwrap
    stream.write_all(response.as_bytes()).unwrap();

    // let http_request: Vec<_> = buf_reader
    //     // split the request by '\n'
    //     .lines()
    //     // check each line for error crash if error
    //     // normally you would add error handling instead of unwrap
    //     .map(|result| result.unwrap())
    //     // streams end with two consecutive '\n' so this will stop reading when that happens
    //     .take_while(|line| !line.is_empty())
    //     // return as a collection or Vec<_>
    //     .collect();
}
