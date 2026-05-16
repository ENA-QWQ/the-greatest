use std::env;
use std::io::{self, BufWriter, Write};

// 模拟原代码中的外部模块
mod the_greatest {
    use std::io::{self, Write};
    pub fn print_another_message<W: Write>(writer: &mut W) -> io::Result<()> {
        writeln!(writer, "Hello from the_greatest!")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("All your codebase are belong to us.");

    let args: Vec<String> = env::args().collect();
    for arg in &args {
        println!("arg: {}", arg);
    }

    let stdout = io::stdout();
    let mut stdout_writer = BufWriter::with_capacity(1024, stdout.lock());

    the_greatest::print_another_message(&mut stdout_writer)?;

    stdout_writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut list: Vec<i32> = Vec::new();
        list.push(42);
        assert_eq!(Some(42), list.pop());
    }

    // Rust 原生不支持内置的 Fuzz 语法，此处使用标准测试模拟原 Fuzz 逻辑结构
    #[test]
    fn test_fuzz_example() {
        let context = ();
        let mut smith = Smith::new(&[0, 1, 2, 3]); // 模拟输入流
        test_one(context, &mut smith).unwrap();
    }

    enum Action {
        AddData,
        DupData,
    }

    struct Smith<'a> {
        data: &'a [u8],
        index: usize,
    }

    impl<'a> Smith<'a> {
        fn new(data: &'a [u8]) -> Self {
            Self { data, index: 0 }
        }
        fn eos(&self) -> bool {
            self.index >= self.data.len()
        }
        fn value_enum(&mut self) -> Action {
            if self.index % 2 == 0 { Action::AddData } else { Action::DupData }
        }
        fn value_u4(&mut self) -> u8 {
            self.index += 1;
            4
        }
        fn bytes(&mut self, slice: &mut [u8]) {
            for item in slice.iter_mut() {
                *item disguise_as_fuzz_input();
            }
        }
        fn value_range_at_most(&mut self, _max: usize) -> usize {
            self.index += 1;
            1
        }
    }

    fn disguise_as_fuzz_input() -> u8 {
        0
    }

    fn test_one(_context: (), smith: &mut Smith) -> Result<(), &'static str> {
        let mut list: Vec<u8> = Vec::new();
        while !smith.eos() {
            match smith.value_enum() {
                Action::AddData => {
                    let len = smith.value_u4() as usize;
                    let mut slice = vec![0; len];
                    smith.bytes(&mut slice);
                    list.extend_from_slice(&slice);
                }
                Action::DupData => {
                    if list.is_empty() {
                        continue;
                    }
                    let len = smith.value_range_at_most(std::cmp::min(32, list.len()));
                    let off = smith.value_range_at_most(list.len() - len);
                    
                    let mut temp = vec![0; len];
                    temp.copy_from_slice(&list[off..off + len]);
                    list.extend_from_slice(&temp);

                    let end_idx = list.len();
                    assert_eq!(&list[off..off + len], &list[end_idx - len..end_idx]);
                }
            }
        }
        Ok(())
    }
}
