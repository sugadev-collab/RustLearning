use std::env::Args;
mod quest;
use crate::quest::Quest;

mod messages;
mod storage_access;
use crate::quest::validate;
enum Commands {
    Help,
    List,
    Add(String, String, String),
    Start(String),
    Complete(String),
    Summary,
}
fn command_identify(arg: &mut Args) -> Result<Commands, String> {
    let command = match arg.nth(1) {
        Some(commnad) => commnad,
        None => "".to_string(),
    };
    if "help".eq(&command) {
        return Ok(Commands::Help);
    } else if "list".eq(&command) {
        return Ok(Commands::List);
    } else if "summary".eq(&command) {
        return Ok(Commands::Summary);
    } else if command.starts_with("start") {
        let st = arg.next();

        return match st {
            Some(quest_no) => Ok(Commands::Start(quest_no.to_string())),
            None => Err("After 'start' command a quest no is expected.".to_string()),
        };
    } else if command.starts_with("complete") {
        let st = arg.nth(2);

        return match st {
            Some(quest_no) => Ok(Commands::Complete(quest_no.to_string())),
            None => Err("After 'complete' command a quest no is expected.".to_string()),
        };
    } else if command.starts_with("add") {
        let tokens: Vec<String> = arg
            .into_iter()
            .enumerate()
            .map(|(i, token)| if i != 0 && i != 1 { Some(token) } else { None })
            .filter(|token_op| !token_op.is_none())
            .map(|token_op| token_op.unwrap_or_default())
            .collect();

        let title = match tokens.get(1) {
            Some(title) => title,
            None => return Err("The add comment require 'title', 'xp' ".to_string()),
        };

        let xp = match tokens.get(2) {
            Some(xp) => xp,
            None => return Err("The add comment require 'xp' ".to_string()),
        };

        let note = match tokens.get(3) {
            Some(note) => note,
            None => "",
        };

        return Ok(Commands::Add(
            title.to_string(),
            xp.to_string(),
            note.to_string(),
        ));
    }
    Err(format!(
        "Unknown command: {} , try 'cargo run -- help'.",
        command
    ))
}

fn help() {
    println!(
        "[Supported commands]:

        [Command]                                   [Required behavior]

        cargo run -- help                           Print the available commands and expected arguments.
        cargo run -- list                           Print every quest with a one-based quest number, status, title, XP, and note fallback.
        cargo run -- add <title> <xp> [note]        Add a new planned quest after validating all supplied data.
        cargo run -- add <title> <xp>               (Without Note supported.)
        cargo run -- start <quest-number>           Change a planned quest to active.
        cargo run -- complete <quest-number>        Change an active quest to completed.
        cargo run -- summary                        Print quest counts by status and the total XP from completed quests.

    "
);
}
fn list() -> Option<Vec<Quest>> {
    let path = "/data/quests.db";
    match storage_access::read(path) {
        Ok(quests_str_result) => Some(
            quests_str_result
                .into_iter()
                .filter_map(|quest_str_result| {
                    let quest_result = match quest_str_result {
                        Ok(quest_str) => validate::new(
                            quest_str.title,
                            quest_str.xp,
                            quest_str.note,
                            quest_str.status,
                        ),
                        Err(e) => {
                            println!("Error: {}", e);
                            Err(e)
                        }
                    };

                    let quest = match quest_result {
                        Ok((quest, message)) => {
                            if !message.is_empty() {
                                println!("{}", message);
                            };
                            Some(quest)
                        }

                        Err(e) => {
                            println!("Error: {}", e);
                            None
                        }
                    };

                    quest
                })
                .collect::<Vec<Quest>>(),
        ),
        Err(read_err) => {
            println!("Error: {}", read_err);
            None
        }
    }
}

fn execute_command(command: Commands) {
    let path = "data/quests.db";
    match command {
        Commands::Help => help(),
        Commands::List => {
            if let Some(quests) = list() {
                quest::print_list(&quests);
            }
        }
        Commands::Add(title, xp, note) => {
            match validate::new(title, xp, note, Some("Planned".to_string())) {
                Ok((quest, _message)) => {
                    let vec_quest = vec![quest];
                    match storage_access::write(vec_quest, path) {
                        Ok(message) => println!("{}", message),
                        Err(e) => println!("Error: {}", e),
                    };
                }
                Err(e) => println!("Error: {}", e),
            };
        }
        Commands::Start(quest_no) => {
            match quest_no.parse::<usize>() {
                Ok(no) => match list() {
                    Some(mut quests) => match quests.get_mut(no - 1) {
                        Some(mut quest) => match validate::active(&mut quest) {
                            Ok(message) => {
                                println!("{}", message);
                                match storage_access::write(quests, path) {
                                    Ok(message) => println!("{}", message),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            Err(e) => println!("Error: {}", e),
                        },
                        None => println!("Could not get quest with no: {}", no),
                    },
                    None => println!("No Quests in DB."),
                },
                Err(e) => println!("Parsing err for no: {} and {}", quest_no, e),
            };
        }
        Commands::Complete(quest_no) => {
            match quest_no.parse::<usize>() {
                Ok(no) => match list() {
                    Some(mut quests) => match quests.get_mut(no - 1) {
                        Some(mut quest) => match validate::complete(&mut quest) {
                            Ok(message) => {
                                println!("{}", message);
                                match storage_access::write(quests, path) {
                                    Ok(message) => println!("{}", message),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            Err(e) => println!("Error: {}", e),
                        },
                        None => println!("Could not get quest with no: {}", no),
                    },
                    None => println!("No Quests in DB."),
                },
                Err(e) => println!("Parsing err for no: {} and {}", quest_no, e),
            };
        }
        Commands::Summary => match list() {
            Some(quests) => {
                quest::summary(&quests);
            }
            None => println!("No quest exist."),
        },
    }
}

fn main() {
    let mut command = std::env::args();

    let identified_command = match command_identify(&mut command) {
        Ok(command) => command,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    execute_command(identified_command);

    // ====== Temprory call ========
}
