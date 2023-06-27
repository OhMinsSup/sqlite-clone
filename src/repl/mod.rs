use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{MatchingBracketValidator, Validator};
use rustyline::validate::{ValidationContext, ValidationResult};
use rustyline::{CompletionType, Config, Context, EditMode};
use rustyline_derive::{Completer, Helper};

// REPL Helper Struct with all functionalities
#[derive(Completer, Helper)]
pub struct REPLHelper {
    pub validator: MatchingBracketValidator,
    pub colored_prompt: String,
    pub hinter: HistoryHinter,
    pub highlighter: MatchingBracketHighlighter,
}

impl REPLHelper {
    // Default constructor
    pub fn new() -> Self {
        REPLHelper {
            // completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        }
    }
}

// 힌트 제공을 담당하는 트레잇 구현
impl Hinter for REPLHelper {
    type Hint = String;

    // 커서 위치로 현재 편집된 줄을 가져와야 하는 문자열을 반환합니다.
    // 표시되거나 사용자가 현재 입력한 텍스트에 대한 힌트가 없는 경우 None
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

// 현재 입력 버퍼가 유효한지 여부를 결정하는 특성을 구현합니다.
// Rustyline은 이 특성이 제공하는 메서드를 사용하여 Enter 키를 눌렀는지 여부를 결정합니다.
// 현재 편집 세션을 종료하고 현재 라인 버퍼를 호출자에게 반환합니다.
// Editor::readline 또는 변형.
impl Validator for REPLHelper {
    // 현재 편집된 입력을 받아 입력 여부를 나타내는 ValidationResult를 반환합니다.
    // 결과에 대해 표시할 옵션 메시지와 함께 유효 여부입니다.
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        use ValidationResult::{Incomplete, /*Invalid,*/ Valid};
        let input = ctx.input();
        // let result = if !input.starts_with("SELECT") {
        //     Invalid(Some(" --< Expect: SELECT stmt".to_owned()))
        // } else
        let result = if input.eq(".exit") {
            Valid(None)
        } else if !input.ends_with(';') {
            Incomplete
        } else {
            Valid(None)
        };
        Ok(result)
    }

    // 입력하는 동안 또는 사용자가 Enter 키를 누를 때만 유효성 검사를 수행할지 여부를 구성합니다.
    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

// ANSI 색상으로 구문 강조 표시를 구현합니다.
impl Highlighter for REPLHelper {
    // 프롬프트를 받고 강조 표시된 버전(ANSI 색상 포함)을 반환합니다.
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    // 힌트를 받아 강조 표시된 버전(ANSI 색상 포함)을 반환합니다.
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    // 현재 편집된 줄을 커서 위치로 가져오고 강조 표시된 버전(ANSI 색상 포함)을 반환합니다.
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    // 특정 문자를 입력하거나 특정 문자 아래로 커서를 이동할 때 줄을 강조 표시해야 하는지 알려줍니다.
    // 문자 삽입 또는 커서 이동 시 새로 고침을 최적화하기 위해 사용합니다.
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

// Returns a Config::builder with basic Editor configuration
pub fn get_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build()
}
