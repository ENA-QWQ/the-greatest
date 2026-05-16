use std::io::{self, Write};

pub fn print_another_message<W: Write>(writer: &mut W) -> io::Result<()> {
    writeln!(writer, "Run `zig build test` to run the tests.")
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add_functionality() {
        assert_eq!(add(3, 7), 10);
    }
}
