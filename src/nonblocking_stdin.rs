use std::io::BufRead;
use tokio::sync::mpsc;

/// For interactive use cases, tokio's documentation (`tokio::io::Stdin`) advise to
/// do blocking io on a dedicated thread.
pub struct Stdin {
    rx: mpsc::Receiver<String>,
}

pub fn stdin() -> Stdin {
    let (tx, rx) = mpsc::channel(8);

    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        while let Some(Ok(line)) = stdin.lock().lines().next() {
            if tx.blocking_send(line).is_err() {
                break;
            }
        }
    });

    Stdin { rx }
}

impl Stdin {
    pub async fn read_line(&mut self) -> Option<String> {
        self.rx.recv().await
    }
}
