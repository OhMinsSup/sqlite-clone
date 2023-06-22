use std::io::{self, Write};
use std::mem;
use std::ptr;

// Constants
const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
const TABLE_MAX_ROWS: usize = 100;
const ROW_SIZE: usize = mem::size_of::<u32>() * 3 + COLUMN_USERNAME_SIZE + COLUMN_EMAIL_SIZE;
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_PAGES: usize = TABLE_MAX_ROWS / ROWS_PER_PAGE;

// Enums
enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    SyntaxError,
    UnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
}

// Structs
struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    fn new(id: u32, username: &str, email: &str) -> Row {
        let mut username_bytes = [0; COLUMN_USERNAME_SIZE];
        let mut email_bytes = [0; COLUMN_EMAIL_SIZE];

        for (i, c) in username.chars().enumerate() {
            username_bytes[i] = c as u8;
        }

        for (i, c) in email.chars().enumerate() {
            email_bytes[i] = c as u8;
        }

        Row {
            id,
            username: username_bytes,
            email: email_bytes,
        }
    }
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

struct Table {
    num_rows: usize,
    pages: Vec<Option<Vec<u8>>>,
}

// Functions
fn new_table() -> Table {
    Table {
        num_rows: 0,
        pages: vec![None; TABLE_MAX_PAGES],
    }
}

// Serialize a row to a byte buffer
fn serialize_row(source: &Row, destination: &mut [u8; ROW_SIZE]) {
    unsafe {
        let id_ptr = source.id.to_le_bytes().as_ptr();
        let username_ptr = source.username.as_ptr();
        let email_ptr = source.email.as_ptr();

        ptr::copy_nonoverlapping(id_ptr, destination.as_mut_ptr(), mem::size_of::<u32>());
        ptr::copy_nonoverlapping(
            username_ptr,
            destination.as_mut_ptr().add(mem::size_of::<u32>()),
            COLUMN_USERNAME_SIZE,
        );
        ptr::copy_nonoverlapping(
            email_ptr,
            destination
                .as_mut_ptr()
                .add(mem::size_of::<u32>() + COLUMN_USERNAME_SIZE),
            COLUMN_EMAIL_SIZE,
        );
    }
}

// Deserialize a row from a byte buffer
fn deserialize_row(source: &[u8; ROW_SIZE]) -> Row {
    let id = u32::from_le_bytes([source[0], source[1], source[2], source[3]]);

    let mut username = [0; COLUMN_USERNAME_SIZE];
    username.copy_from_slice(&source[4..36]);

    let mut email = [0; COLUMN_EMAIL_SIZE];
    email.copy_from_slice(&source[36..291]);

    Row {
        id,
        username,
        email,
    }
}

// Find the position of a row in a table
fn row_slot(table: &mut Table, row_num: usize) -> *mut u8 {
    let page_num = row_num / ROWS_PER_PAGE;
    if page_num >= TABLE_MAX_PAGES {
        panic!("Table overflow");
    }
    if table.pages.get(page_num).is_none() {
        table.pages.resize_with(page_num + 1, || None);
    }
    if table.pages[page_num].is_none() {
        table.pages[page_num] = Some(vec![0; PAGE_SIZE]);
    }
    let row_offset = row_num % ROWS_PER_PAGE;
    let byte_offset = row_offset * ROW_SIZE;
    unsafe {
        table.pages[page_num]
            .as_mut()
            .unwrap()
            .as_mut_ptr()
            .add(byte_offset)
    }
}

// Main function to execute statement on the table
fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement.statement_type {
        StatementType::Insert => {
            if table.num_rows >= TABLE_MAX_ROWS {
                return ExecuteResult::TableFull;
            }
            let row_to_insert = statement.row_to_insert.as_ref().unwrap();
            let slot = row_slot(table, table.num_rows);
            serialize_row(row_to_insert, unsafe {
                &mut *(slot as *mut [u8; ROW_SIZE])
            });
            table.num_rows += 1;
            ExecuteResult::Success
        }
        StatementType::Select => {
            for i in 0..table.num_rows {
                let row =
                    deserialize_row(unsafe { &*(row_slot(table, i) as *const [u8; ROW_SIZE]) });
                println!("({}, {:?}, {:?})", row.id, row.username, row.email);
            }
            ExecuteResult::Success
        }
    }
}

// ExecuteResult enum
enum ExecuteResult {
    Success,
    TableFull,
}

// Main function
fn main() {
    let mut table = new_table();
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().starts_with('.') {
            match do_meta_command(&input) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input.trim());
                    continue;
                }
            }
        }

        let statement = match prepare_statement(&input) {
            Ok(statement) => statement,
            Err(PrepareResult::Success) => {
                println!("Syntax error. Could not parse statement '{}'", input.trim());
                continue;
            }
            Err(PrepareResult::SyntaxError) => {
                println!("Syntax error. Could not parse statement '{}'", input.trim());
                continue;
            }
            Err(PrepareResult::UnrecognizedStatement) => {
                println!("Unrecognized keyword at start of '{}'", input.trim());
                continue;
            }
        };

        let result = execute_statement(&statement, &mut table);
        match result {
            ExecuteResult::Success => println!("Executed."),
            ExecuteResult::TableFull => println!("Error: Table full."),
        }
    }
}

// Meta command handling
fn do_meta_command(input: &str) -> MetaCommandResult {
    if input.trim() == ".exit" {
        std::process::exit(0);
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}

// Statement preparation
fn prepare_statement(input: &str) -> Result<Statement, PrepareResult> {
    // Insert statement
    if input.trim().starts_with("insert") {
        let statement_parts: Vec<&str> = input.trim().split(' ').collect();
        if statement_parts.len() != 4 {
            return Err(PrepareResult::SyntaxError);
        }
        let id = statement_parts[1]
            .parse::<u32>()
            .map_err(|_| PrepareResult::SyntaxError)?;
        let username = statement_parts[2];
        let email = statement_parts[3];
        let row = Row::new(id, username, email);

        Ok(Statement {
            statement_type: StatementType::Insert,
            row_to_insert: Some(row),
        })
    // Select statement
    } else if input.trim() == "select" {
        Ok(Statement {
            statement_type: StatementType::Select,
            row_to_insert: None,
        })
    } else {
        Err(PrepareResult::UnrecognizedStatement)
    }
}
