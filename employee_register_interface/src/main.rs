use std::{collections::HashMap, io::Write};

const HELP_STRING: &str = "Available commands:\n\
                           add <name> to <department>   Add employee to department\n\
                           list <department>            List employees in department\n\
                           list all                     List all employees\n\
                           help                         Print this help text\n\
                           quit                         Quit";

fn main() {
    println!("Welcome to Employee Register Interface!");
    println!("{}", HELP_STRING);

    let mut departments_with_employees = HashMap::new();

    loop {
        print!("> ");
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return,
        };
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Reading line failed");

        let command_split: Vec<&str> = command.trim().split_whitespace().collect();
        if command_split.len() == 4 && command_split[0] == "add" && command_split[2] == "to" {
            let name = command_split[1];
            let department = command_split[3];
            departments_with_employees
                .entry(department.to_string())
                .or_insert(Vec::new())
                .push(name.to_string());
            println!("Operation successful!");
            continue;
        }
        if command_split.len() == 2 && command_split[0] == "list" {
            let department = command_split[1];
            if department == "all" {
                let mut employees = departments_with_employees
                    .values()
                    .flatten()
                    .collect::<Vec<&String>>();
                employees.sort();
                employees
                    .iter()
                    .for_each(|employee| println!("{}", employee));
            } else {
                match departments_with_employees.get(department) {
                    Some(employees) => {
                        let mut employees = employees.clone();
                        employees.sort();
                        employees
                            .iter()
                            .for_each(|employee| println!("{}", employee));
                    }
                    None => println!("Department {} doesn't exist.", department),
                }
            }
            continue;
        }
        match command.trim() {
            "help" => println!("{}", HELP_STRING),
            "quit" => return,
            _ => println!("Invalid command.\n{}", HELP_STRING),
        }
    }
}
