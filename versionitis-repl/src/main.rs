use versionitis::Repo;
use std::io::Write;
use std::io::Read;
use versionitis::traits::TrackPackages;

struct RepoRepl {
    repo: Repo,
    feedback: Option<String>,
}

impl RepoRepl {
    fn new() -> Self {
        Self {
            repo: Repo::new(),
            feedback: None,
        }
    }

    fn await_user(&self) {
        print!("\nPress enter to continue...");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let mut inp: [u8;1] = [0];
        match io::stdin().read_line(&mut input) {
            _ => {}
        }
    }

    fn print_cmds(&mut self) {
        print!("{}[2J", 27 as char);
        println!("\nVersionitis - REPL");
        println!("(d) - display repo");
        println!("(l) - load repo from file");
        println!("(v) - add version");
        println!("(q) - quit");

        if let Some(fb) = &self.feedback {
            println!("FEEDBACK:{}",fb);
        }
        // set feedback back to None
        self.feedback.take();
        print!("\n(choice):");
        io::stdout().flush().unwrap();
    }

    fn display_repo(&self) {
        print!("{}[2J", 27 as char);
        println!("{:#?}", self.repo);
        self.await_user();
    }

    fn add_version(&mut self) -> bool {
        print!("{}[2J", 27 as char);
        print!("(name-version):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop(); //remove \n
                let pieces: Vec<&str> = input.split("-").collect();
                if pieces.len() != 2 { return false; }
                match self.repo.add_version(pieces[0], pieces[1]) {
                    Ok(_) => {
                        println!("added {}", input);
                        self.await_user();
                    }
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        self.await_user();
                        return false;
                    }
                };

            }
            _ => {}
        }

        true
    }

    fn handle_results(&mut self, input: &str) -> bool {
        match input {
            "d" => {self.display_repo();}
            "l" => {println!("l - load repo from file");}
            "v" => {self.add_version();}
            "q" => {
                println!("quiting");
                return true;
            }
            _ => {println!("invalid choice: '{}'", input);}
        }
        false
    }

    fn run(&mut self) {
    loop {
            self.print_cmds();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input.pop();
                    if self.handle_results(input.as_str()) { break };
                }
                Err(error) => self.feedback = Some(error.to_string()),
            }
        }
    }
}


/*
 let package_name = "fred";
    // make a mess
    repo.add_version_unchecked(package_name, "0.2.0");
    repo.add_version_unchecked(package_name, "0.1.0");
    // duplicate insert
    repo.add_version_unchecked(package_name, "0.1.0");
    repo.add_version_unchecked(package_name, "0.2.1");
    repo.add_version_unchecked(package_name, "0.3.0");
    // out of order insert
    repo.add_version_unchecked(package_name, "0.2.3");
    // clean up
    repo.dedup_sort();
    let s = serde_yaml::to_string(&repo).unwrap();
    println!("{}",s);
 */
use std::io;

fn main() {
    let mut repo = RepoRepl::new();
    repo.run();
}