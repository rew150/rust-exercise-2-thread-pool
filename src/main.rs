mod pool;

use std::{io::{self, Read, Write}, net::{TcpStream, TcpListener}, thread, time::Duration};

use crate::pool::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    println!("start");
    let mut buf = [0u8; 128];
    stream.set_read_timeout(None).unwrap();
    loop {
        let read = stream.read(&mut buf).unwrap();
        if read == 0 {
            break;
        }
        stream.write(&buf[0..read]).unwrap();
    }
    println!("wait 3 sec");
    thread::sleep(Duration::from_secs(3));
    println!("finish");
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888")?;
    let threadpool = ThreadPool::new(2);
    println!("listening");
    for stream in listener.incoming() {
        let stream = stream?;
        threadpool.launch(move || {
            handle_client(stream);
        });
    }
    Ok(())
}
