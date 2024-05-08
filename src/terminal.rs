use std::io::{Result, StdinLock, StdoutLock};
use crate::{keyboard::AllKeyEvents, text::{Text, TextLines}};

pub trait Terminal {
    /// Read key events (in order) to a buffer.
    fn read_keys(&mut self, buffer: &mut Vec<AllKeyEvents>) -> Result<usize>;

    /// Push text without a newline character at the end.
    fn push_text(&mut self, text: &Text) -> Result<()>;

    /// Push text with a newline character at the end (`\n`)
    fn push_line(&mut self, line: &Text) -> Result<()>;

    /// Push multiple [`Text`] elements separated by the newline character (`\n`)
    fn push_lines(&mut self, lines: &TextLines) -> Result<()>;
}

pub(crate) struct TermionTerminal<'a> {
    pub stdin: StdinLock<'a>,
    pub stdout: StdoutLock<'a>,
}

impl Terminal for TermionTerminal<'_> {
    fn read_keys(&mut self, buffer: &mut Vec<AllKeyEvents>) -> Result<usize> {
        todo!()
    }

    fn push_text(&mut self, text: &Text) -> Result<()> {
        todo!()
    }

    fn push_line(&mut self, line: &Text) -> Result<()> {
        todo!()
    }

    fn push_lines(&mut self, lines: &TextLines) -> Result<()> {
        todo!()
    }
}