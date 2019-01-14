use versionitis::Repo;
use versionitis::traits::TrackPackages;
use versionitis::errors::VersionitisError;
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;

type Feedback = Option<Result<String, String>>;

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
        println!("(w) - write repo to file");
        println!("(v) - add version");
        println!("(q) - quit");

        if let Some(fb) = &self.feedback {
            println!("(ERROR):{}",fb);
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

    fn add_version(&mut self) -> Result<bool, VersionitisError> {
        print!("{}[2J", 27 as char);
        print!("(name-version):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop(); //remove \n
                let pieces: Vec<&str> = input.split("-").collect();
                if pieces.len() != 2 {
                    return Err(
                        VersionitisError::AddVersionError(
                            format!("malformed package name: {}", input)
                        )
                    );
                }

                match self.repo.add_version(pieces[0], pieces[1]) {
                    Ok(_) => {
                        println!("added {}", input);
                        self.await_user();
                    }
                    Err(e) => {
                        self.feedback = Some(e.to_string());
                        //self.await_user();
                        return Err(VersionitisError::AddVersionError(e.to_string()));
                    }
                };

            }
            Err(e) => {
                self.feedback = Some(e.to_string());
                return Err(VersionitisError::AddVersionError(e.to_string()));
            }
        }

        Ok(true)
    }

    fn load_from_file(&mut self) -> Result<(),versionitis::errors::VersionitisError> {
        print!("{}[2J", 27 as char);
        print!("\n(file):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                if !Path::new(&input).exists() {
                    self.feedback = Some(format!("path:'{}' does not exist on disk", input));
                    return Err(VersionitisError::NonExtantFileError(format!("path:'{}' doesn't exist", input)));
                }
                println!("loading {}", input);
                let f = std::fs::File::open(input)?;
                let r: Repo = serde_yaml::from_reader(f)?;
                self.repo = r;
                Ok(())
            }
            Err(error) => {self.feedback = Some(error.to_string()); Err(VersionitisError::IoError(error))},
        }
    }
    fn write_repo(&self) -> Result<(), VersionitisError> {
        print!("{}[2J", 27 as char);
        print!("\n(output):");
        io::stdout().flush().unwrap();
        let mut output = String::new();
        match io::stdin().read_line(&mut output) {
            Ok(_) => {
                output.pop();
                let f = File::create(output)?;
                let r = serde_yaml::to_writer(f, &self.repo)?;
                Ok(r)
            }
            Err(e) => {
                Err(VersionitisError::IoError(e))
            }
        }

    }

    fn handle_results(&mut self, input: &str) -> Result<bool,VersionitisError> {
        match input {

            "d" => {
                self.display_repo();
            }

            "l" => {
                println!("l - load repo from file");
                match self.load_from_file() {
                    Err(e) => { println!("Error loading file");}
                    Ok(_) => { }
                }
            }

            "w" => {
                self.write_repo();

            }

            "v" => {
                self.add_version()?;

            }
            "q" => {
                println!("quiting");
                return Ok(true);
            }
            _ => {println!("invalid choice: '{}'", input);}
        }
        // we return Ok(false) to indicate that we are not quiting
        Ok(false)
    }

    fn run(&mut self) {
    loop {
            self.print_cmds();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input.pop();
                    if let Ok(quitout) = self.handle_results(input.as_str())
                   { if quitout == true { break }};
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

fn main() {
    let mut repo = RepoRepl::new();
    repo.run();
}