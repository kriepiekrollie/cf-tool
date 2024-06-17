use crate::cli;
use crate::client;
use crate::config;
use crate::cf;

pub fn test_code() {
    println!("Testing code.");
}

pub fn generate_file(_args: &cli::TemplateArgs) {
    println!("Generating file.");
}

pub fn parse(args: &cli::ContestArgs) {
    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    let config_file_path = config_dir.join("config.json");
    let session_file_path = config_dir.join("session.json");
    let mut conf = config::Config::load_or_new(&config_file_path).unwrap();
    let mut clint = client::Client::load_or_new(&session_file_path);
    // Definitely move some stuff out of this function.
    clint.parse_sample_testcases(&args, &conf.cf_root);
    // clint is doing some heavy lifting
}

pub fn login() {
    // TODO: Check if user is already logged in and then do stuff
    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    let session_file_path = config_dir.join("session.json");
    let mut clint = client::Client::load_or_new(&session_file_path);
    if clint.login(cli::prompt_login_details()) {
        clint.save(&session_file_path);
    }
    // Who is clint?
}

pub fn template_add() {
    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    let config_file_path = config_dir.join("config.json");
    let mut conf = config::Config::load_or_new(&config_file_path).unwrap();

    conf.add_template(cli::prompt_new_template());
    conf.save(&config_file_path).unwrap();
}

pub fn template_delete(args: &cli::TemplateArgs) {
    config::delete_template(&args).unwrap();
}

pub fn template_set_default(args: &cli::TemplateArgs) {
    config::set_default_template(&args);
}

pub fn submit() {
    let current_dir = std::env::current_dir().unwrap();
    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    let config_file_path = config_dir.join("config.json");
    let session_file_path = config_dir.join("session.json");

    let mut conf = config::Config::load_or_new(&config_file_path).unwrap();
    let mut client = client::Client::load_or_new(&session_file_path);

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
