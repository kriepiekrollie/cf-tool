use crate::cf;
use crate::cli;
use crate::client;
use crate::config;
use crate::files;
pub mod template;

pub fn test_code() {
    println!("Testing code.");
}

pub fn generate_file(_args: &cli::TemplateArgs) {
    println!("Generating file.");
}

pub fn parse(args: &cli::ContestArgs) {
    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();
    let mut clint = client::Client::load_or_new(&files::session_file_path());
    // Definitely move some stuff out of this function.
    clint.parse_sample_testcases(&args, &conf.cf_root);
    // clint is doing some heavy lifting
}

pub fn login() {
    // TODO: Check if user is already logged in and then do stuff
    let mut clint = client::Client::load_or_new(&files::session_file_path());
    if clint.login(cli::prompt_login_details()) {
        clint.save(&files::session_file_path());
    }
    // Who is clint?
}

pub fn submit() {
    let current_dir = std::env::current_dir().unwrap();
    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();
    let mut clint = client::Client::load_or_new(&files::session_file_path());

    // Get problem information from current directory.
    // TODO: Perhaps give ability to overwrite this functionality with command-line args?
    let problem_info = cf::ProblemInfo::from_path(&current_dir, &conf.cf_root).unwrap();

    // let code_info = cli::prompt_code()
    let current_dir = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(current_dir).unwrap();

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            let file_name = file_name.to_lowercase();
            let file_name = file_name.as_str();
            let file_name = file_name.split('.').collect::<Vec<&str>>();
            let file_name = file_name[file_name.len()-1];
            let file_name = file_name.to_string();

            println!("{}", file_name);
            let sussy: Vec<_> = conf.templates
                .iter()
                .filter(|x| x.suffix.contains(&file_name))
                .collect();

            println!("{:?}", sussy);
            // if conf.templates.contains_key(file_name) {
            //     println!("Using template: {}", file_name);
            // } else {
            //     println!("Using default template.");
            // }
        }
        // if let Ok(file_type) = pathprintln!();
    }
    // let submission = cli::get_submission(&conf).unwrap();
    // client.submit_code(problem_info);
}

pub fn source()  {
    let url = "https://www.github.com/";
    open::that(url)
        .unwrap_or_else(|_| {
            println!("Failed to open link in browser: {}", url)
        });
}
