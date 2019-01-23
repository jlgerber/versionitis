use versionitis::Repo;
use versionitis::traits::TrackPackages;
use versionitis::errors::VersionitisError;
use std::fs::File;
use std::io::{self, Write,};
use std::path::Path;
use versionitis::manifest::Manifest;

type Feedback = Option<Result<String, String>>;

struct RepoRepl {
    repo: Repo,
    feedback:Feedback,
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
        match io::stdin().read_line(&mut input) {
            _ => {}
        }
    }

    fn print_cmds(&mut self) {
        print!("{}[2J", 27 as char);
        println!(  "\n==================");
        println!("Versionitis - REPL");
        println!(  "==================\n");
        println!("Options:\n");
        println!("(d) - display repo");
        println!("(l) - list versions for package");
        println!("(p) - list packages");
        println!("(r) - read repo from file");
        println!("(w) - write repo to file");
        println!("(v) - add version");
        println!("(m) - serialize manifest test");
        println!("(n) - read manifest from disk");
        println!("(q) - quit");

        match &self.feedback {

            Some(Ok(feedback)) => {
                println!("\n[STATUS] {}", feedback);
            }
            Some(Err(feedback)) => {
                println!("\n[ERROR] {}", feedback);
            }
            _ => {}
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
                        self.feedback = Some(Err(e.to_string()));
                        //self.await_user();
                        return Err(VersionitisError::AddVersionError(e.to_string()));
                    }
                };

            }
            Err(e) => {
                self.feedback = Some(Err(e.to_string()));
                return Err(VersionitisError::AddVersionError(e.to_string()));
            }
        }

        Ok(true)
    }

    fn list_packages(&self) {
        print!("{}[2J", 27 as char);
        println!("========");
        println!("Packages");
        println!("========\n");
        let mut packs = self.repo.packages.iter().map(|(k,_)| k.to_string()).collect::<Vec<String>>();
        packs.sort();

        for package in &packs {
            println!("{}", package);
        }
        self.await_user();
    }

    fn load_from_file(&mut self) -> Result<(),versionitis::errors::VersionitisError> {
        print!("{}[2J", 27 as char);
        let path = std::env::current_dir()?;
        print!("\n(file [CWD:{}] ):", path.to_str().unwrap());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                if !Path::new(&input).exists() {
                    self.feedback = Some(Err(format!("path:'{}' does not exist on disk", input)));
                    return Err(VersionitisError::NonExtantFileError(format!("path:'{}' doesn't exist", input)));
                }
                self.feedback = Some(Ok(format!("loaded: {}", input)));
                let f = std::fs::File::open(input)?;
                let r: Repo = serde_yaml::from_reader(f)?;
                self.repo = r;
                Ok(())
            }
            Err(error) => {
                self.feedback = Some(Err(error.to_string()));
                Err(VersionitisError::IoError(error.to_string()))
            }
        }
    }
    fn write_repo(&mut self) -> Result<(), VersionitisError> {
        print!("{}[2J", 27 as char);
        print!("\n(output):");
        io::stdout().flush().unwrap();
        let mut output = String::new();
        match io::stdin().read_line(&mut output) {
            Ok(_) => {
                output.pop();
                let f = File::create(output.as_str())?;
                let r = serde_yaml::to_writer(f, &self.repo)?;
                self.feedback=Some(Ok(format!("wrote repo to: {}", output)));
                Ok(r)
            }
            Err(e) => {
                Err(VersionitisError::IoError(e.to_string()))
            }
        }
    }

    fn serialize_manifest(&self) -> Result<(),VersionitisError>  {
        use versionitis::manifest::{Manifest};
        use versionitis::version_number_interval::VersionNumberInterval;
        use versionitis::interval::Range::*;
        type VI=VersionNumberInterval;
        let mut manifest = Manifest::new("fred-1.0.0");
        let interval1 = VI::from_range(&Single("0.1.0")).unwrap();
        let interval2 = VI::from_range(&HalfOpen("0.1.0", "1.0.0")).unwrap();
        let interval3 = VI::from_range(&Open("0.1.0", "1.0.0")).unwrap();
        manifest.add_dependency("foo", interval1).unwrap();
        manifest.add_dependency("bar", interval2).unwrap();
        manifest.add_dependency("bla", interval3).unwrap();

        let f = File::create("/tmp/manifest.yaml")?;
        serde_yaml::to_writer(f, &manifest)?;
        Ok(())
    }

    fn deserialize_manifest(&mut self) -> Result<Manifest,VersionitisError>  {
        print!("{}[2J", 27 as char);
        let path = std::env::current_dir()?;
        print!("\n(file [CWD:{}] ):", path.to_str().unwrap());
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                if !Path::new(&input).exists() {
                    self.feedback = Some(Err(format!("path:'{}' does not exist on disk", input)));
                    return Err(VersionitisError::NonExtantFileError(format!("path:'{}' doesn't exist", input)));
                }
                self.feedback = Some(Ok(format!("loaded: {}", input)));
                let f = std::fs::File::open(input)?;
                let r: Manifest = serde_yaml::from_reader(f)?;
                Ok(r)
            }

            Err(error) => {
                Err(VersionitisError::IoError(error.to_string()))
            }
        }
    }

    fn handle_results(&mut self, input: &str) -> Result<bool, VersionitisError> {
        match input {

            "d" => {
                self.display_repo();
            }

            "p" => {
                self.list_packages();
            }

            "r" => {
                println!("r - fead repo from file");
                self.load_from_file()?;
            }

            "w" => {
                self.write_repo()?;

            }

            "v" => {
                self.add_version()?;

            }

            "m" => {
                match self.serialize_manifest() {
                    Err(error) => {
                        self.feedback = Some(Err(error.to_string()));
                    }
                    Ok(_) => {}
                }

            }

            "n" =>  {
                let result = self.deserialize_manifest();
                match result {
                    Ok(manifest) => {
                        println!("{:#?}", manifest);
                        self.await_user();
                    }
                    Err(error) => {
                        self.feedback = Some(Err(error.to_string()));
                    }
                }
            }

            "q" => {
                println!("quiting");
                return Ok(true);
            }

            _ => {self.feedback = Some(Err(format!("invalid choice: '{}'", input)));}
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
                Err(error) => self.feedback = Some(Err(error.to_string())),
            }
        }
    }
}




fn main() {
    let mut repo = RepoRepl::new();
    repo.run();
}