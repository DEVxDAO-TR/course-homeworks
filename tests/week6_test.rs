
#[cfg(test)]
mod week6_tests {
    use course_homeworks::week6;

    use std::time::Duration;    
    use std::thread;
    use std::net::{TcpStream, Shutdown};
    use std::io::{self, Read, Write};

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
        thread::spawn(|| { week6::start_server().unwrap() });

        let client1 = thread::spawn(client);
        let client2 = thread::spawn(client);

        let c1_response = client1.join().unwrap().unwrap();
        let c2_response = client2.join().unwrap().unwrap();

        assert_eq!(c1_response, c2_response);
    }
}
