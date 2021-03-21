use std::io::Read;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    ops::Deref,
    path::PathBuf,
    process::{ChildStdin, ChildStdout, Command, Stdio},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

pub struct RawClient {
    pub bin_path: PathBuf,

    pub sender: Option<Sender<Vec<u8>>>,
    pub receiver: Option<Receiver<Vec<u8>>>,
}

impl RawClient {
    pub fn new(bin_path: PathBuf) -> Self {
        return Self {
            bin_path,

            sender: None,
            receiver: None,
        };
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::new(self.bin_path.as_os_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = cmd
            .stdin
            .take()
            .ok_or("language server has no stdin".to_owned())?;

        let stdout = cmd
            .stdout
            .take()
            .ok_or("language server has no stdout".to_owned())?;

        let (s1, r1) = channel();
        spawn_read_thread(stdout, s1);
        self.receiver = Some(r1);

        let (s2, r2) = channel();
        spawn_write_thread(stdin, r2);
        self.sender = Some(s2);

        Ok(())
    }

    pub fn send(&self, data: Vec<u8>) {
        match &self.sender {
            Some(sender) => {
                sender.send(data).ok();
                ()
            }
            None => (),
        }
    }
}

fn spawn_read_thread(stdout: ChildStdout, sender: Sender<Vec<u8>>) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new(); // save some allocations
        loop {
            // read headers
            let content_length = parse_headers(&mut reader, &mut line);

            let len = match content_length {
                Some(len) => len,
                None => {
                    eprintln!("Lsp error: No Content-Length.");
                    break;
                }
            };

            let mut data = vec![0u8; len];

            match reader.read_exact(&mut data) {
                Err(e) => {
                    eprintln!("Lsp error couldn't read exact: {}", e);
                    break;
                }
                _ => (),
            }

            sender
                .send(data)
                .map_err(|e| eprintln!("Error while sending data in lsp: {}", e))
                .ok();
        }
    });
}

fn parse_headers(reader: &mut BufReader<ChildStdout>, line: &mut String) -> Option<usize> {
    let mut content_length: Option<usize> = None;

    loop {
        line.clear();

        match reader.read_line(line) {
            Ok(0) => {
                eprintln!("Lsp done");
                return None;
            }
            Err(e) => {
                eprintln!("Lsp error while reading: {}", e);
                return None;
            }
            _ => {
                // empty line => headers ended
                if line.len() <= 2 {
                    break;
                }

                // parse header
                // TODO use str.split_once when it's available
                let mut it = line.splitn(2, ": ");
                let key = it.next().unwrap_or("");
                let value = it.next().unwrap_or("").trim();

                if key == "Content-Length" {
                    match value.parse() {
                        Ok(value) => content_length = Some(value),
                        Err(e) => {
                            eprintln!("Lsp error parsing Content-Length: {}", e);
                            return None;
                        }
                    }
                }
            }
        }
    }

    content_length
}

fn spawn_write_thread(mut stdin: ChildStdin, receiver: Receiver<Vec<u8>>) {
    thread::spawn(move || loop {
        match receiver.recv() {
            Ok(msg) => {
                println!("{:?}", String::from_utf8(msg.clone()));

                //Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n
                let header = format!("Content-Length: {}\r\n\r\n", msg.len());

                match stdin
                    .write(header.into_bytes().deref())
                    .and_then(|_| stdin.write(&*msg))
                {
                    Err(e) => println!("error writing to language server: {}", e),
                    _ => (),
                }
            }
            _ => break,
        }
    });
}
