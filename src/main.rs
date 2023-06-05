use std::io::{stdin, stdout, Write};

// https://cstack.github.io/db_tutorial/

enum PrepareResult {
    Success,               // 성공
    UnrecognizedStatement, // 인식 할 수 없는 명령어
}

enum MetaCommandResult {
    Success,             // 성공
    UnrecognizedCommand, // 인식 할 수 없는 명령어
}

enum StatementType {
    Insert, // 삽입
    Select, // 선택
}

struct Statement {
    statement_type: StatementType,
}

// part.2
// sqlite의 "프론트 엔드"는 문자열을 구문 분석하고 바이트 코드라는 내부 표현을 출력하는 SQL 컴파일러입니다.
// - 각 부분의 복잡성 감소 (예: 가상 머신은 구문 오류에 대해 걱정하지 않음)
// - 일반 쿼리를 한번 컴파일하고 성능 향상을 위해 바이트 코드를 캐싱 할 수 있음

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

    // read_line 메서드를 사용하여 사용자 입력을 읽어들입니다.
    // 입력 버퍼의 마지막 문자를 제거하고, buffer 필드에 저장합니다.
    fn read_input(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        self.input_length = input.len() as i64 - 1;
        self.buffer = input.trim_end().to_string();
    }

    // close 메서드는 입력 버퍼를 지우는 역할을 합니다. buffer 필드를 지우고, input_length 필드를 0으로 설정합니다.
    fn close(&mut self) {
        self.buffer.clear();
        self.input_length = 0;
    }
}

fn do_meta_command(input_buffer: &mut InputBuffer) -> MetaCommandResult {
    // 사용자가 ".exit" 명령을 입력하면 루프가 종료되고 프로그램이 종료됩니다. 그렇지 않으면 콘솔에 오류 메시지가 출력됩니다.
    match input_buffer.buffer.as_str() {
        ".exit" => {
            input_buffer.close();
            std::process::exit(0);
        }
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}

// sql 컴파일러는 사용자 입력을 읽고, 입력이 인식되지 않으면 오류 메시지를 출력합니다.
fn prepare_statement(input_buffer: &mut InputBuffer, statement: &mut Statement) -> PrepareResult {
    let statement_str = input_buffer.buffer.as_str();
    if statement_str.starts_with("insert") {
        statement.statement_type = StatementType::Insert;
        return PrepareResult::Success;
    }
    if statement_str.starts_with("select") {
        statement.statement_type = StatementType::Select;
        return PrepareResult::Success;
    }
    PrepareResult::UnrecognizedStatement
}

// 가상 머신은 바이트 코드를 실행하는 데 사용되는 가상 머신입니다.
fn execute_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do a select."),
    }
}

// print! 매크로를 사용하여 db >  문자열을 출력하고,
// stdout을 플러시하여 출력을 플러시합니다.
fn print_prompt() {
    print!("db > ");
    stdout().flush().unwrap();
}

// 사용자 입력을 읽고, 입력이 인식되지 않으면 오류 메시지를 출력하는 간단한 데이터베이스 셸을 Rust로 구현한 것입니다.
fn main() {
    // main 함수는 InputBuffer 인스턴스를 초기화하고,
    // 사용자 입력이 인식될 때까지 루프를 반복합니다.
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        if input_buffer.buffer.starts_with('.') {
            match do_meta_command(&mut input_buffer) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input_buffer.buffer.trim());
                    continue;
                }
            }
        }

        let mut statement = Statement {
            statement_type: StatementType::Insert,
        };
        match prepare_statement(&mut input_buffer, &mut statement) {
            PrepareResult::Success => (),
            PrepareResult::UnrecognizedStatement => {
                println!(
                    "Unrecognized keyword at start of '{}'",
                    input_buffer.buffer.trim()
                );
                continue;
            }
        }

        execute_statement(&statement);
        println!("Executed.");
    }
}
