/*
use std::io::{self, BufRead, Write};

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    /*
     * parse the command line arugmnet
     */
    let yaml = load_yaml!("../arguments.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut contest = atcoder::Contest::new();

    if matches.is_present("interactive") {
        interactive_mode(&mut contest);
        return;
    }

    assert!(false, "not yet implemented");
}
*/

//use std::io::{self, BufRead, Read, Stdin, Write};
use std::io::{self, BufRead, Write};

mod contest;
mod extracter;
mod task;

struct ExecInfo {
    file_name: String,
}

fn eval_command(contest: &mut contest::Contest, exec_info: &mut ExecInfo, command_line: String) -> Result<bool, String> {
    let command: Vec<_> = command_line.split(' ').collect();
    match command[0] {
        "contest_title" | "c" => {
            // there are argument
            if command.len() != 2 {
                match &contest.title {
                    Some(title) => {
                        println!("{}", title);
                        Ok(true)
                    }
                    None => Err(
                        "contest_name requires a argument.\ne.g. > contest_name abc125".to_string(),
                    ),
                }
            } else {
                //setting contest
                println!("setting...");

                let title = command[1];
                let is_exist = contest.set_title(title.to_string());

                if !is_exist {
                    Err("The contest title is invalid".to_string())
                } else {
                    Ok(true)
                }
            }
        }
        "show_task" | "s" => {
            if command.len() == 2 {
                let task_num = command[1].parse::<usize>();
                if let Ok(num) = task_num {
                    contest.current_task_index = num;
                    // TODO
                    if let Some(t) = contest.get_task() {
                        // TODO
                        let show_ret = t.show();
                        Ok(true)
                    } else {
                        Err("TODO".to_string())
                    }
                } else {
                    Err("invalied argument".to_string())
                }
            } else {
                if let Some(t) = contest.get_task() {
                    // TODO
                    t.show();
                    Ok(true)
                } else {
                    Err("TODO".to_string())
                }
            }
        }
        "exec_file" | "e" => {

            if command.len() != 2 {
                exec_info.file_name = command[1].to_string();
                Ok(true)
            }
            else{
                Err("exec_file requires a argument.\n.e.g. > exec_file ./a.out".to_string())
            }
        }
        "bye" | "b" => Ok(false),
        _ => Err("invalid command".to_string()),
    }
}

//fn interactive_mode<R: BufRead>(read: &mut R) {
fn interactive_mode(contest: &mut contest::Contest) {
    let mut exec_info = ExecInfo { file_name: "./a.out".to_string() };
    loop {
        // show the prompt
        print!("> ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        let line = iterator.next().unwrap().unwrap();
        // throw away second line or later.

        // TODO
        let is_continue = match eval_command(contest, &mut exec_info, line) {
            Ok(cnt) => cnt,
            Err(why) => {
                println!("failed\n{}", why);
                true
            }
        };

        if !is_continue {
            println!("bye");
            break;
        }
    }
}

fn main() {
    let mut contest = contest::Contest::new();
    interactive_mode(&mut contest);
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::io::Cursor;

    //#[test]
    //fn stdin() {
    //    let mut stdin = io::stdin();
    //    interactive_mode(&mut io::stdin());
    //}

    //#[test]
    //fn string_in() {
    //    let input_data = b"help".to_vec();
    //    let mut input = Cursor::new(input_data);
    //    interactive_mode(&mut input)
    //}

    #[test]
    fn eval_get_title_failed() {
        let mut contest = contest::Contest::new();
        let line = "c\n".to_string();
    let mut exec_info = ExecInfo { file_name: "./a.out".to_string() };
        let result = eval_command(&mut contest, &mut exec_info, line);

        match result {
            Ok(_) => {
                assert!(false);
            }
            Err(_why) => {}
        }
    }

    #[test]
    fn eval_set_title() {
        let mut contest = contest::Contest::new();
        let line = "c agc012".to_string();
    let mut exec_info = ExecInfo { file_name: "./a.out".to_string() };
        let result = eval_command(&mut contest, &mut exec_info, line);

        match result {
            Ok(_) => {}
            Err(_why) => {
                assert!(false);
            }
        }
    }

#[test]
    fn eval_show_task() {
        let mut contest = contest::Contest::new();
    let mut exec_info = ExecInfo { file_name: "./a.out".to_string() };
        let line = "c agc012".to_string();
        eval_command(&mut contest, &mut exec_info, line).unwrap();
        eval_command(&mut contest, &mut exec_info, "s".to_string()).unwrap();
    }

#[test]
    fn eval_exit() {
    let mut exec_info = ExecInfo { file_name: "./a.out".to_string() };
        let mut contest = contest::Contest::new();
        let line = "bye".to_string();
        eval_command(&mut contest, &mut exec_info, line).unwrap();
    }
}
