use std::{
    env,
    net::TcpListener,
};

mod request_handler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_args = if args.len() > 2 {
        println!("Detect directory: {}", args[2]);
        args[2].clone()
    } else {
        println!("No directory specified, using current directory");
        env::current_dir().unwrap().to_str().unwrap().to_string()
    };
    let dir = std::path::Path::new(&dir_args).to_path_buf();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let dir = dir.clone();
                std::thread::spawn(
                    move || {
                        let request = request_handler::Request::from_stream(&mut _stream);
                        request_handler::handle_request(request, _stream, dir);
                    }
                );
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
