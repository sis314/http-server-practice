use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let (mut rd, mut wr) = io::split(socket);
    let mut buf = vec![0; 1024];
    rd.read_buf(&mut buf).await.unwrap();
    let recv_data = String::from_utf8_lossy(&buf[..]);

    let request = recv_data
        .chars()
        .skip_while(|c| *c == '\0')
        .take_while(|c| *c != '\r')
        .collect::<String>();
    let requests = request.split_whitespace().collect::<Vec<&str>>();

    let response = match requests[0] {
        "GET" => responce_get(requests[1]).await,
        _ => "HTTP/1.1 501 Not Implemented\r\n".to_string(),
    };

    wr.write(response.as_bytes()).await.unwrap();
    wr.flush().await.unwrap();
}

async fn responce_get(mut path: &str) -> String {
    use tokio::fs::File;
    if path == "/" {
        path = "/index.html";
    }
    let path = &(".".to_string() + path);
    if let Ok(mut file) = File::open(path).await {
        let mut body = String::new();
        if file.read_to_string(&mut body).await.is_err() {
            "HTTP/1.1 404 NotFound".to_string()
        } else {
            format!("HTTP/1.1 200 OK\r\n\r\n{}", body)
        }
    } else {
        println!("path {} not found", path);
        "HTTP/1.1 404 NotFound".to_string()
    }
}
