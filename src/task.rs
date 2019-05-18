use crate::extracter::atcoder;
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct SampleIO {
    pub input: String,
    pub output: String,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub title: String,
    pub problem_statement: Option<String>,
    pub sample_ios: Option<Vec<SampleIO>>,
    pub io_style: Option<String>,
    lang: atcoder::Lang,
    pub url: String,
}

impl Task {
    pub fn new(title: String, url: String, lang: atcoder::Lang) -> Task {
        Task {
            title: title,
            problem_statement: None,
            sample_ios: None,
            io_style: None,
            lang: lang,
            url: url,
        }
    }

    pub fn complete(&mut self) {
        if self.problem_statement == None || self.sample_ios == None || self.io_style == None {
            //TODO read data ans set
            atcoder::set_task_info(self);
        }
    }

    pub fn show(&self) -> bool {
        let p_state = if let Some(statement) = &self.problem_statement {
            statement
        } else {
            return false;
        };

        println!("title:\n{}", self.title);
        println!("-----");
        println!("statement:\n{}", p_state);
        println!("-----");
        if let Some(style) = &self.io_style {
            print!("{}", style);
        } else {
            return false;
        }
        println!("-----");

        if let Some(sample_vec) = &self.sample_ios {
            println!("sample I/Os:");
            for sample in sample_vec {
                print!("{}{}", sample.input, sample.output);
                println!("-----");
            }
        } else {
            return false;
        }

        true
    }

    // this function returns a failed sample index;
    pub fn run_test(&self, sample_num: i32, cmd: &String) -> Result<Vec<usize>, String> {

        let mut failed_list = vec![];

        if let Some(samples) = &self.sample_ios {
            // run with all samples
            if sample_num == -1 {
                for (i, sample) in samples.iter().enumerate() {
                    let mut process = Command::new(cmd)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .ok()
                        .expect("failed to start the program");
                    process
                        .stdin
                        .as_mut()
                        .unwrap()
                        .write(sample.input.as_bytes())
                        .unwrap();
                    println!("--- sample{:<02} ---\n", i);

                    let output = process.wait_with_output().unwrap();
                    let result_str = String::from_utf8(output.stdout).unwrap();
                    if result_str == sample.output {
                        println!("passed!");
                    } else {
                        println!("failed");
                        print!("correct answer is\n{}", sample.output);
                        print!("your answer is \n{}", result_str);
                        failed_list.push(i);
                    }
                }
            }
            else{
                let index = sample_num as usize;
                let sample = &samples[index];
                let mut process = Command::new(cmd)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .ok()
                    .expect("failed to start the program");
                process
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write(sample.input.as_bytes())
                    .unwrap();

                let output = process.wait_with_output().unwrap();
                let result_str = String::from_utf8(output.stdout).unwrap();
                if result_str == sample.output {
                    println!("passed!\n");
                } else {
                    println!("failed\n");
                    println!("correct answer is\n{}", sample.output);
                    println!("your answer is \n{}", result_str);
                    failed_list.push(index);
                }
            }

            Ok(failed_list)
        }
        else
        {
            Err("cannot find any samples".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let domain = "atcoder.jp".to_string();
        let path = "/contests/abc125/tasks".to_string();

        let result = atcoder::get_tasks(&domain, &path);
        let lang_ja = atcoder::Lang::Ja;

        let mut task_result = vec![];
        if let Some(tasks) = result {
            for task in tasks {
                task_result.push(Task::new(task.title, task.link, lang_ja));
            }
        } else {
            assert!(false);
        }

        let answer = vec![
            Task::new(
                "A: Biscuit Generator".to_string(),
                "https://atcoder.jp/contests/abc125/tasks/abc125_a".to_string(),
                atcoder::Lang::Ja,
            ),
            Task::new(
                "B: Resale".to_string(),
                "https://atcoder.jp/contests/abc125/tasks/abc125_b".to_string(),
                atcoder::Lang::Ja,
            ),
            Task::new(
                "C: GCD on Blackboard".to_string(),
                "https://atcoder.jp/contests/abc125/tasks/abc125_c".to_string(),
                atcoder::Lang::Ja,
            ),
            Task::new(
                "D: Flipping Signs".to_string(),
                "https://atcoder.jp/contests/abc125/tasks/abc125_d".to_string(),
                atcoder::Lang::Ja,
            ),
        ];
        assert_eq!(answer.len(), task_result.len());
        for i in 0..answer.len() {
            assert_eq!(answer[i], task_result[i]);
        }
    }
}

/*

use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
#[derive(Debug)]
pub struct SampleIO {
    pub input: String,
    pub output: String,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub problem_statement: Option<String>,
    pub sample_ios: Option<Vec<SampleIO>>,
    pub io_style: Option<String>,
    lang: String,
    url: String,
}

impl Task {
    pub fn new(title: String, domain: String, path: String) -> Task {
        let url = &format!("https://{0}{1}", domain, path);
        let lang = "ja".to_string();

        //let mut response = reqwest::get(url).unwrap();
        //TODO

        Task {
            title: title,
            problem_statement: None,
            sample_ios: None,
            io_style: None,
            lang: lang,
            url: url.to_string(),
        }
    }
    //pub fn new(html_body: String, lang: String) -> Task {
    //    let document = Html::parse_document(&html_body);
    //    let lang = &format!("span.lang-{}", lang);

    //    let sel_lang = Selector::parse(lang).unwrap();;
    //    let task_statement_root = document.select(&sel_lang).next().unwrap();

    //    let (sample_ios, io_style) = Task::get_samples(&task_statement_root);

    //    Task {
    //        problem_statement: Task::get_problem_statement(&task_statement_root),
    //        sample_ios: sample_ios,
    //        io_style: io_style,
    //        lang: lang.to_string(),
    //    }
    //}

    fn get_problem_statement(html: &ElementRef) -> String {
        let selector_div1 = Selector::parse("div.part").unwrap();
        let selector_p = Selector::parse("p").unwrap();

        let problem_v: Vec<_> = html
            .select(&selector_div1)
            .flat_map(|item| item.select(&selector_p))
            .map(|item| item.inner_html())
            .collect();

        problem_v
            .concat()
            .replace("<var>", "")
            .replace("</var>", "")
            .replace("<br>", "")
            .replace("<code>", "")
            .replace("</code>", "")
    }

    fn get_samples(html: &ElementRef) -> (Vec<SampleIO>, String) {
        let sel_sample = Selector::parse("div.part").unwrap();
        let sel_section = Selector::parse("section").unwrap();
        let sel_pre = Selector::parse("pre").unwrap();

        let samples: Vec<_> = html
            .select(&sel_sample)
            .flat_map(|item| item.select(&sel_section))
            .flat_map(|item| item.select(&sel_pre))
            .map(|item| item.inner_html())
            .collect();

        let io_style = &samples[0].replace("<var>", "").replace("</var>", "");
        let mut iter = samples[1..].iter();

        let mut io_samples = Vec::new();
        while let Some(input) = iter.next() {
            if let Some(output) = iter.next() {
                let sample = SampleIO {
                    input: input.to_string(),
                    output: output.to_string(),
                };
                io_samples.push(sample);
            } else {
                assert!(false, "I/Os must be coresponded.");
            }
        }

        (io_samples, io_style.to_string())
    }
}
*/
