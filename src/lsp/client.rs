use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    ops::Deref,
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc::Receiver,
    thread,
};

pub struct Client {
    pub path: PathBuf,
}

impl Client {
    pub fn new(path: PathBuf) -> Client {
        return Client { path };
    }

    pub fn run(&self, msg_recv: Receiver<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::new(self.path.as_os_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = cmd
            .stdin
            .take()
            .ok_or("language server has no stdin".to_owned())?;

        let stdout = cmd
            .stdout
            .take()
            .ok_or("language server has no stdout".to_owned())?;

        // Read
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        print!("{}", &line);
                    }
                    Err(_) => (),
                };
            }
        });

        // Write
        thread::spawn(move || loop {
            match msg_recv.recv() {
                Ok(msg) => match stdin.write(msg.deref()) {
                    Err(e) => println!("error writing to language server: {}", e),
                    _ => (),
                },
                _ => break,
            }
        });

        Ok(())
    }
}
