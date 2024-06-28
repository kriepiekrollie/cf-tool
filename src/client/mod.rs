mod serde_cookies;
mod scrape;

mod postbody;
use postbody::PostBody;

use crate::{
    config::LoginDetails,
    cli::ContestArgs,
    cf::{
        ProblemInfo,
        ContestInfo,
        SubmissionInfo,
    },
};

use rand::prelude::*;
use std::{
    fs,
    io,
    path::PathBuf,
    collections::HashMap,
    sync::Arc,
    process,
};

use serde::{Serialize, Deserialize};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

use colored::Colorize;
use anyhow::{Context, Result, bail};

// This is what gets stored in .config/cf-tool/session.json
// To be used with serde_json..
#[derive(Serialize, Deserialize)]
struct SessionInfo {
    #[serde(with = "serde_cookies")]
    pub cookies: Arc<CookieStoreMutex>, // <- I made a (de)serializer for this
    pub ftaa: String,
    pub bfaa: String,
    pub tta: String,
}

impl SessionInfo {
    pub fn new() -> Self {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        Self {
            cookies: cookie_store,
            ftaa: Self::gen_ftaa(),
            bfaa: Self::gen_bfaa(),
            tta: Self::gen_tta(),
        }
    }
    fn gen_ftaa() -> String {
        let mut rng = rand::thread_rng();
        std::iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
            .take(18)
            .collect::<String>()
            .to_lowercase()
    }
    // This is what xalanq did... I'm not sure why but if it works it works.
    fn gen_bfaa() -> String {
        String::from("f1b3f18c715565b589b7823cda7448ce")
    }
    fn gen_tta() -> String {
        String::from("176")
    }
}

// Struct which is exported and does the stuff...
pub struct Client {
    client: reqwest::blocking::Client,
    session: SessionInfo,
}

impl Client {
    pub fn new() -> Result<Self> {
        let session = SessionInfo::new();
        let client = reqwest::blocking::Client::builder()
            .cookie_provider(Arc::clone(&session.cookies)).build()
            .with_context(|| "Failed to create HTTP client.")?;
        Ok(Self {
            client,
            session,
        })
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let reader = fs::File::open(path)
            .map(io::BufReader::new).with_context(||
            format!("Failed to open file for reading: {:?}", path))?;
        let session: SessionInfo = serde_json::from_reader(reader).with_context(||
            format!("Failed to read cookies from file: {:?}", path))?;
        let client = reqwest::blocking::Client::builder()
            .cookie_provider(Arc::clone(&session.cookies)).build()
            .with_context(|| "Failed to create HTTP client.")?;
        Ok(Self {
            client,
            session,
        })
    }

    pub fn load_or_new(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            Ok(Self::load(path)?)
        } else {
            Ok(Self::new()?)
        }
    }

    pub fn write(&self, path: &PathBuf) -> Result<()> {
        if let Some(par) = path.parent() {
            fs::create_dir_all(&par).with_context(|| 
                format!("Failed to create config directory: {:?}", path))?;
        }
        let writer = fs::File::create(path)
            .map(io::BufWriter::new).with_context(||
            format!("Failed to open file for writing: {:?}", path))?;
        serde_json::to_writer_pretty(writer, &self.session).with_context(|| 
            format!("Failed to write session information to file: {:?}", path))?;
        Ok(())
    }

    pub fn login(&mut self, details: LoginDetails) -> Result<bool> {
        // Codeforces login page.
        let url = "https://codeforces.com/enter";

        *self = Client::new()?;

        // Get csrf by doing initial response
        let response = self.client.get(url).send()
            .with_context(|| format!("Failed to make request to {}", url))?.text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        let html = scraper::Html::parse_document(&response);
        let csrf_token = scrape::csrf(&html)
            .with_context(|| "Failed to get CSRF from HTML")?;

        let post_body = serde_qs::to_string(&PostBody::Login {
            csrf_token,
            ftaa: self.session.ftaa.clone(),
            bfaa: self.session.bfaa.clone(),
            tta: self.session.tta.clone(),
            handle_or_email: details.handle_or_email,
            password: details.password,
            remember: 
                if details.remember { 
                    String::from("yes") 
                } else { 
                    String::from("no") 
                },
        }).with_context(|| "Failed to create query string for post request.")?;

        // Make a POST request to the url, which redirects us to the home page.
        let response = self.client.post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_body).send()
            .with_context(|| format!("Failed to make request to {}", &url))?.text()
            .with_context(|| format!("Failed to get response from {}", &url))?;

        let html = scraper::Html::parse_document(&response);

        if let Some(handle) = scrape::handle(&html) {
            println!("{}", format!("You have successfully logged in as {}!", 
                    handle).green().bold());
            Ok(true)
        } else {
            println!("{}", "Failed to log in :(".red().bold());
            Ok(false)
        }
    }

    pub fn parse_sample_testcases(&self, args: &ContestArgs) -> Result<HashMap<String, Vec<(String, String)>>> {
        // TODO: Make the arguments to this some nicer type?
        let contest_type = args.contest_type();

        let url = format!("https://codeforces.com/{}/{}/problems", 
            &contest_type, &args.contest_id);

        // I don't feel like this print statement belongs here...
        println!("{}", format!("Parsing {} {} from {}", &contest_type, 
                &args.contest_id, &url.underline()).blue().bold());
        
        let response = self.client.get(url.clone()).send()
            .with_context(|| format!("Failed to make request to {}", url))?
            .text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        // First, check that what we got is even the correct thing.
        if response.contains("Codeforces.showMessage(\"No such contest\");") {
            bail!("{}", "No such contest.".red().bold());
        }

        let html = scraper::Html::parse_document(&response);
        scrape::sample_tests(&html)
    }

    pub fn submit_code(&self, problem_info: &ProblemInfo, source: &String, language_id: &u8) -> Result<SubmissionInfo> {

        let url = format!("https://codeforces.com/{}/{}/submit", 
            problem_info.contest.typ, problem_info.contest.id);

        // println!("{}", url);

        // Get csrf by doing initial response
        let response = self.client.get(&url).send()
            .with_context(|| format!("Failed to make request to {}", url))?.text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        let html = scraper::Html::parse_document(&response);
        let csrf_token = scrape::csrf(&html)
            .with_context(|| "Failed to get CSRF from HTML")?;

        // First, check that what we got is even the correct thing.
        if response.contains("Codeforces.showMessage(\"No such contest\");") {
            bail!("{}", "No such contest.".red().bold());
        }

        let handle = scrape::handle(&html)
            .with_context(|| "Login failed. Try logging in with cf login".red().bold())?;
        println!("{}", format!("Submitting with account: {}", handle).cyan().bold());

        let problem_index = scrape::problem_index(&html, &problem_info.id)
            .with_context(|| "Failed to get problem index. Does this problem exist?")?;

        let post_body = serde_qs::to_string(&PostBody::Submit {
            csrf_token,
            ftaa: self.session.ftaa.clone(),
            bfaa: self.session.bfaa.clone(),
            tta: self.session.tta.clone(),
            contest_id: problem_info.contest.id.clone(),
            problem_index,
            language_id: *language_id,
            source: source.to_string(),
            tab_size: 4,
            source_file: String::from(""),
        }).with_context(|| "Failed to create query string for post request.")?;
        
        // println!("{:?}", post_body);

        // Make a POST request to the url, which redirects us to the home page.
        let response = self.client.post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_body).send()
            .with_context(|| format!("Failed to make request to {}", url))?.text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        if response.contains("You have submitted exactly the same code before") {
            println!("{}", "You have submitted exactly the same code before".red().bold());
            process::exit(1);
        }

        let html = scraper::Html::parse_document(&response);
        // println!("{:?}", response);

        Ok(scrape::latest_submission_info(&html)
            .with_context(|| "Failed to get submission information.")?)
    }

    pub fn get_submission(&self, contest_info: &ContestInfo, submission_id: &String) -> Result<SubmissionInfo> {
        let url = format!("https://codeforces.com/{}/{}/my", contest_info.typ, contest_info.id);

        let response = self.client.get(&url)
            .send()
            .with_context(|| format!("Failed to make request to {}", url))?
            .text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        let html = scraper::Html::parse_document(&response);

        Ok(scrape::submission_info(&html, &submission_id)
            .with_context(|| "Failed to get submission info.")?)
    }

    #[allow(dead_code)]
    fn debug_test_logged_in(&self) -> Result<bool> {
        let url = "https://codeforces.com";

        // make a simple request to the home page.
        let response = self.client.get(url).send()
            .with_context(|| format!("Failed to make request to {}", url))?.text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        let html = scraper::Html::parse_document(&response);

        let store = self.session.cookies.lock().expect("Poisoned mutex :(");
        for cookie in store.iter_any() {
            println!("{:?}", cookie);
        }

        // Try to find the handle.
        let handle = scrape::handle(&html);

        if let Some(handle) = &handle {
            println!("{}", format!("You are indeed logged in as {}!", handle)
                .green().bold());
            Ok(true)
        } else {
            println!("{}", "Failed to log in.".red().bold());
            Ok(false)
        }
    }
}
