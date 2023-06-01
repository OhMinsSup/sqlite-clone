// https://cstack.github.io/db_tutorial/
use std::io;
use std::io::Write;

// InputBuffer 구조체는 사용자 입력을 저장하는 String과 입력 길이를 추적하는 input_length 필드를 포함합니다.
// new 메서드는 새로운 InputBuffer 인스턴스를 생성합니다.
// read_input 메서드는 stdin에서 사용자 입력을 읽어들입니다.
struct InputBuffer {
    buffer: String,
    input_length: i64,
}

impl InputBuffer {
    fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            input_length: 0,
        }
    }

    // print! 매크로를 사용하여 db >  문자열을 출력하고,
    // stdout을 플러시하여 출력을 플러시합니다.
    // read_line 메서드를 사용하여 사용자 입력을 읽어들입니다.
    // 입력 버퍼의 마지막 문자를 제거하고, buffer 필드에 저장합니다.
    fn read_input(&mut self) {
        let mut input = String::new();
        print!("db > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        self.input_length = input.len() as i64 - 1;
        self.buffer = input.trim_end().to_string();
    }

    // close 메서드는 입력 버퍼를 지우는 역할을 합니다. buffer 필드를 지우고, input_length 필드를 0으로 설정합니다.
    fn close(&mut self) {
        self.buffer.clear();
        self.input_length = 0;
    }
}

// 사용자 입력을 읽고, 입력이 인식되지 않으면 오류 메시지를 출력하는 간단한 데이터베이스 셸을 Rust로 구현한 것입니다.
fn main() {
    // main 함수는 InputBuffer 인스턴스를 초기화하고,
    // 사용자 입력이 인식될 때까지 루프를 반복합니다.
    // 사용자가 ".exit" 명령을 입력하면 루프가 종료되고 프로그램이 종료됩니다. 그렇지 않으면 콘솔에 오류 메시지가 출력됩니다.
    let mut input_buffer = InputBuffer::new();

    loop {
        input_buffer.read_input();

        match input_buffer.buffer.as_str() {
            ".exit" => {
                input_buffer.close();
                std::process::exit(0);
            }
            _ => {
                println!("Unrecognized command '{}'", input_buffer.buffer);
            }
        }
    }
}
