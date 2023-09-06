use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};


pub struct ReadServer {
    stdout: BufReader<ChildStdout>,
    // stderr: BufReader<ChildStderr>,
}

impl ReadServer {
    // 读取数据
    pub fn read_data(&mut self, data: &mut String) -> std::io::Result<usize> {
        self.stdout.read_line(data)
    }
}

pub struct WriteServer {
    stdin: ChildStdin,
}

impl WriteServer {
    // 写入数据
    pub fn write_data(&mut self, data: String) -> std::io::Result<()> {
        self.stdin.write_all(data.as_bytes())
    }
}

pub struct ShellServer {
    cmd: Child,
}

impl ShellServer {
    // 初始化shell服务(传入待执行的命令)
    pub fn build(cmd: &str, args: Vec<&str>) -> ShellServer {
        let cmd = Command::new(cmd).args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        ShellServer { cmd }
    }

    // 分割为一个读取数据，一个写入数据
    pub fn split(&mut self) -> (WriteServer, ReadServer) {
        return (
            WriteServer {
                stdin: self.cmd.stdin.take().unwrap(),
            },
            ReadServer {
                stdout: BufReader::new(self.cmd.stdout.take().unwrap()),
                // stderr: BufReader::new(self.cmd.stderr.take().unwrap()),
            },
        );
    }
}
