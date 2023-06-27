extern crate clap;

mod repl;

use repl::{get_config, REPLHelper};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{crate_version, App};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    // CLI 애플리케이션의 인수가 없는 기본 구현
    // 지금은 도움말을 표시하고 향후 구현을 위한 자리 표시자로
    // 작동합니다.
    let _matches = App::new("Rust-SQLite")
        .version("0.0.1")
        .author("Veloss <mins5190@gmail.com>")
        .about("Light version of SQLite developed with Rust")
        .get_matches();
    // 기본 구성으로 Rustyline 시작

    let config = get_config();

    // 새로운 Rustyline Helper 얻기
    let helper = REPLHelper::new();

    // set config 및 설정 헬퍼로 Rustyline Editor 초기화
    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    // 이 메서드는 기록 파일을 메모리에 로드합니다.
    // 없으면 생성
    // TODO: 기록 파일 크기를 확인하고 너무 크면 정리합니다.
    if repl.load_history("history").is_err() {
        println!("No previous history.");
    }

    // 카운터는 사용자 경험을 개선하고 사용자에게 얼마나 많은지 표시하도록 설정됩니다.
    // 그가 실행한 명령.
    let mut count = 1;

    loop {
        if count == 1 {
            // Friendly intro message for the user
            println!(
                "{}{}{}{}{}",
                format!("Rust-SQLite - {}\n", crate_version!()),
                "Enter .exit to quit.\n",
                "Enter .help for usage hints.\n",
                "Connected to a transient in-memory database.\n",
                "Use '.open FILENAME' to reopen on a persistent database."
            );
            //TODO: Get info about application name and version dinamically.
        }

        let p = format!("rust-sqlite | {}> ", count);
        repl.helper_mut().expect("No helper found").colored_prompt =
            format!("\x1b[1;32m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                // println!("Command: {}", line);
                if command.eq(".exit") {
                    break;
                } else {
                    println!(
                        "Error: unknown command or invalid arguments: '{}'. Enter '.help'",
                        &command
                    );
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }

    repl.append_history("history").unwrap();

    Ok(())
}
