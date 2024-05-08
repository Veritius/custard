pub mod keyboard;
pub mod terminal;
pub mod text;

fn main() {
    use std::io::{stdin, stdout};

    let mut terminal = terminal::TermionTerminal {
        stdin: stdin().lock(),
        stdout: stdout().lock(),
    };

    todo!()
}