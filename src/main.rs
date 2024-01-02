use std::{io, process};
use std::io::Write;
use colored::Colorize;
use crate::PrepareResult::PrepareUnrecognizedStatement;
use crate::StatementType::{EmptyStatement, StatementInsert, StatementSelect};

fn print_db_start() {
    println!("{}", "Here is the db-rs, welcome!".yellow());
}

fn print_prompt() {
    print!("{}", "db-rs> ".italic().blue());
}

fn do_meta_commend(input: &String) -> MetaCommandResult {
    let meta_res = match input.as_str() {
        ".exit" => {
            println!("Thank for using, see you!");
            process::exit(0x001);
        }
        _ => {
            println!("{}", "Unrecognized Meta Commend".red());
            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    };
    meta_res
}

fn prepare_statement(input: &String, statement: &mut Statement) -> PrepareResult {
    let input_vec: Vec<&str> = input.split(" ").collect();
    let state = match input_vec[0] {
        "insert" => {
            if input_vec.len() != 4 {
                println!("{}", "Error Commend".red());
                return PrepareUnrecognizedStatement;
            }
            statement.state_type = StatementInsert;
            statement.insert_row.id = input_vec[1].parse::<i32>().unwrap();
            statement.insert_row.username = input_vec[2].to_string();
            statement.insert_row.email = input_vec[3].to_string();
            log::info!("id: {}  username: {}  email: {}", statement.insert_row.id,
                statement.insert_row.username , statement.insert_row.email);
            PrepareResult::PrepareResultSuccess
        }
        "select" => {
            statement.state_type = StatementSelect;
            PrepareResult::PrepareResultSuccess
        }
        _ => {
            statement.state_type = EmptyStatement;
            PrepareUnrecognizedStatement
        }
    };
    state
}

fn execute_statement(statement: &Statement) {
    match statement.state_type {
        StatementInsert => {
            println!("{}", "Do insert here.".green());
        }
        StatementSelect => {
            println!("{}", "Do select here.".green());
        }
        EmptyStatement => {
            println!("{}", "Unrecognized Commend.".red());
        }
    }
}

fn read_input() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");
    String::from(input_buffer.trim())
}

#[derive(PartialEq)]
enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

#[derive(PartialEq)]
enum PrepareResult {
    PrepareResultSuccess,
    PrepareUnrecognizedStatement,
}

#[derive(Debug, PartialEq)]
enum StatementType {
    StatementInsert,
    StatementSelect,
    EmptyStatement,
}

#[derive(Debug, PartialEq)]
struct Statement {
    state_type: StatementType,
    insert_row: Row,
}

#[derive(Debug, PartialEq)]
struct Row {
    id: i32,
    username: String,
    email: String,
}

impl Row {
    fn new() -> Self {
        Row { id: 0, username: String::new(), email: String::new() }
    }
}

impl Statement {
    fn new() -> Self {
        Statement { state_type: EmptyStatement, insert_row: Row::new() }
    }
}

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    print_db_start();
    loop {
        print_prompt();
        io::stdout().flush().unwrap();
        let command = read_input();
        if command.starts_with(".") {
            do_meta_commend(&command);
        }
        let mut state = Statement::new();
        let pre_ste = prepare_statement(&command, &mut state);
        if pre_ste == PrepareResult::PrepareResultSuccess {
            log::info!("Input correct commend {}", command);
        } else {
            log::warn!("Input error commend {}", command);
        }
        execute_statement(&state);
    }
}


