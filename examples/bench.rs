use humanize::humanize_number;
use std::io::{self, BufRead, BufWriter, Write};
fn main() {
    let stdout = std::io::stdout();
    let locked_stdout = stdout.lock();
    let mut buf = BufWriter::new(locked_stdout);
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        writeln!(buf, "{}", humanize_number(&line)).unwrap();
    }
    buf.flush().unwrap();
}
