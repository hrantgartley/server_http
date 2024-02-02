use reqwest::Client;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                let request = String::from_utf8_lossy(&buf[..bytes_read]);
                if request.contains("GET / HTTP/1.1") {
                    let response = generate_html();
                    let response_str = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        response.len(),
                        response
                    );
                    stream
                        .write_all(response_str.as_bytes())
                        .expect("Failed to write to the stream");
                } else {
                    let not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                    stream
                        .write_all(not_found.as_bytes())
                        .expect("Failed to write to the stream");
                }
            }
            Ok(_) | Err(_) => break,
        }
    }
}
async fn validate_html(html: &str) -> Result<bool, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post("https://validator.w3.org/nu/")
        .header("Content-Type", "text/html; charset=utf-8")
        .body(html.to_string())
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(!response_text.contains("error"))
}

fn generate_html() -> String {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Rust Web Server</title>
    </head>
    <body>
        <h1>Hello from Rust</h1>
    </body>
    </html>
    "#;
    html.to_string()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
