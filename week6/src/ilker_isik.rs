use async_std::task;
use async_std::io;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};

/// Process incoming connection
async fn process_connection(stream: TcpStream) -> io::Result<()> {
    let mut reader = stream.clone();
    let mut writer = stream;
    io::copy(&mut reader, &mut writer).await?;

    Ok(())
}


/// Echo server listening on port 22222
pub fn start_server() -> io::Result<()> {
    // Create a listener
    // Wait for completion
    let listener = task::block_on(async {
        TcpListener::bind("127.0.0.1:22222").await
    })?;

    task::spawn(async move {
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream.unwrap();
            task::spawn(async {
                process_connection(stream).await.unwrap();
            });
        }
    });

    Ok(())
}


#[cfg(test)]
mod week6_tests {
    use std::io::{self, Read, Write};
    use std::net::{Shutdown, TcpStream};
    use std::thread;
    use std::time::Duration;

    fn client() -> io::Result<String> {
        let mut stream = TcpStream::connect("127.0.0.1:22222")?;
        thread::sleep(Duration::from_secs(1));

        let mut request = String::from("Merhaba");
        stream.write_all(request.as_bytes())?;

        thread::sleep(Duration::from_secs(1));

        let mut response = [0u8; 128];
        let n = stream.read(&mut response)?;

        stream.shutdown(Shutdown::Both).unwrap();

        request.push_str(std::str::from_utf8(&response[0..n]).unwrap());

        Ok(request)
    }

    #[test]
    fn test_server() {
        // Start the server and wait until it's ready
        super::start_server().unwrap();

        let client1 = thread::spawn(client);
        let client2 = thread::spawn(client);

        let c1_response = client1.join().unwrap().unwrap();
        let c2_response = client2.join().unwrap().unwrap();

        assert_eq!(c1_response, c2_response);
    }
}
