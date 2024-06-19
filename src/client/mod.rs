use crate::config::LoginDetails;
use crate::cli::ContestArgs;
use crate::utils;

use rand::prelude::*;
use std::path::PathBuf;
use std::io::Write;
use std::{io, fs};

use serde::{Serialize, Deserialize};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::sync::Arc;
mod serde_cookies;

use colored::Colorize;
use anyhow::{Context, Result};

// An enum to store the body of a post request.
// To be used with serde_qs.
#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
enum PostBody {
    #[serde(rename = "enter")]
    Login {
        csrf_token: String,
        ftaa: String,
        bfaa: String,
        #[serde(rename = "_tta")]
        tta: String,
        #[serde(rename = "handleOrEmail")]
        handle_or_email: String,
        password: String,
        remember: String,
    },
    #[serde(rename = "submitSolutionFormSubmitted")]
    Submit {
        csrf_token: String,
        ftaa: String,
        bfaa: String,
        #[serde(rename = "_tta")]
        tta: String,
        #[serde(rename = "contestId")]
        contest_id: String,
        #[serde(rename = "submittedProblemIndex")]
        problem_index: String,
        #[serde(rename = "programTypeId")]
        language_id: String,
        source: String,
        #[serde(rename = "tabSize")]
        tab_size: u8,
        #[serde(rename = "sourceFile")]
        source_file: String,
    },
}

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
        let writer = fs::File::create(path)
            .map(io::BufWriter::new).with_context(||
            format!("Failed to open file for writing: {:?}", path))?;
        serde_json::to_writer_pretty(writer, &self.session).with_context(|| 
            format!("Failed to write session information to file: {:?}", path))?;
        Ok(())
    }

    fn find_csrf(html: &scraper::Html) -> Option<String> {
        // Technically there should only be one csrf token but i'm doing this to be safe lol
        let csrf_selector = scraper::Selector::parse("span.csrf-token").unwrap();
        let csrf_tokens = html.select(&csrf_selector);
        for csrf_token in csrf_tokens {
            if let Some(s) = csrf_token.value().attr("data-csrf") {
                return Some(s.to_string());
            }
        }
        None
    }

    fn get_csrf(&self, url: &str) -> Result<String> {
        // Make a get request to the url and find the CSRF token 
        let response = self.client.get(url).send()
            .with_context(|| format!("Failed to make request to {}", url))?.text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        let html = scraper::Html::parse_document(&response);
        let csrf = Self::find_csrf(&html)
            .with_context(|| "Failed to get CSRF from HTML")?;
        Ok(csrf)
    }

    fn find_handle(html: &scraper::Html) -> Option<String> {
        let lang_chooser_selector = scraper::Selector::parse("div.lang-chooser").unwrap();
        let a_selector = scraper::Selector::parse("a").unwrap();
        for lang_choose in html.select(&lang_chooser_selector) {
            for a in lang_choose.select(&a_selector) {
                if let Some(href) = a.value().attr("href") {
                    if href.starts_with("/profile/") {
                        return Some(href.to_string().replace("/profile/", ""));
                    }
                }
            }
        }
        None
    }

    pub fn login(&mut self, details: LoginDetails) -> Result<bool> {
        // Codeforces login page.
        let url = "https://codeforces.com/enter";

        *self = Client::new()?;

        // Get csrf by doing initial response
        let csrf_token = self.get_csrf(url)?;

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

        if let Some(handle) = Self::find_handle(&html) {
            println!("{}", format!("You have successfully logged in as {}!", 
                    handle).green().bold());
            Ok(true)
        } else {
            println!("{}", "Failed to log in :(".red().bold());
            Ok(false)
        }
    }

    pub fn parse_sample_testcases(&self, args: &ContestArgs, root_dir: &PathBuf) -> Result<()> {
        let contest_type = args.contest_type();

        let url = format!("https://codeforces.com/{}/{}/problems", 
            &contest_type, &args.contest_id);

        println!("{}", format!("Parsing {} {} from {}", &contest_type, 
                &args.contest_id, &url.underline()).blue().bold());
        
        let response = self.client.get(url.clone()).send()
            .with_context(|| format!("Failed to make request to {}", url))?
            .text()
            .with_context(|| format!("Failed to get response from {}", url))?;

        // First, check that what we got is even the correct thing.
        if response.contains("Codeforces.showMessage(\"No such contest\");") {
            println!("{}", "No such contest.".red().bold());
            return Ok(());
        }

        let html = scraper::Html::parse_document(&response);

        // TODO: maybe check if we can find a span.countdown (start a race)

        // I'm pretty sure these won't fail.
        let pdiv_selector = scraper::Selector::parse("div.problemindexholder").unwrap();
        let indiv_selector = scraper::Selector::parse("div.input").unwrap();
        let outdiv_selector = scraper::Selector::parse("div.output").unwrap();
        let pre_selector = scraper::Selector::parse("pre").unwrap();

        let contest_dir = root_dir.join(format!("{}", &contest_type))
            .join(&args.contest_id);

        for problemdiv in html.select(&pdiv_selector) {
            let problem_index = problemdiv.value().attr("problemindex")
                .with_context(|| "Failed to parse samples from HTML")?;

            let problem_dir = contest_dir.join(&problem_index.to_lowercase());
            fs::create_dir_all(&problem_dir).with_context(|| 
                format!("Failed to create directory: {:?}", problem_dir))?;

            let mut count: u8 = 0;
            for (idx, indiv) in problemdiv.select(&indiv_selector).enumerate() {
                let path = problem_dir.join(format!("{}.in", idx));
                let pre = indiv.select(&pre_selector).next().with_context(||
                    "Failed to parse samples from HTML.")?;

                let data = pre.text()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");

                let mut writer = fs::File::create(&path)
                    .map(io::BufWriter::new).with_context(||
                    format!("Failed to open file for writing: {:?}", path))?;

                writer.write(data.as_bytes()).with_context(|| 
                    format!("Failed to write to file: {:?}", path))?;
                count += 1;
            }

            for (idx, outdiv) in problemdiv.select(&outdiv_selector).enumerate() {
                let path = problem_dir.join(format!("{}.out", idx));
                let pre = outdiv.select(&pre_selector).next().with_context(||
                    "Failed to parse samples from HTML.")?;

                let data = pre.text()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");

                let mut writer = fs::File::create(&path)
                    .map(io::BufWriter::new).with_context(||
                    format!("Failed to open file for writing: {:?}", path))?;

                writer.write(data.as_bytes()).with_context(|| 
                    format!("Failed to write to file: {:?}", path))?;
            }

            println!("Problem {}: parsed {} sample testcase{}.", 
                problem_index, count, if count == 1 { "" } else { "s" });
        }

        println!("{}", format!("Sample testcases have been stored in {:?}", utils::path_shortest_repr(&contest_dir)).green().bold());
        Ok(())
    }

    // pub fn submit_code(&self, problem_info: cf::ProblemInfo) -> Result<()> {
    //     // let diff = pathdiff::diff_paths(std::env::current_dir().unwrap(), &root_dir).unwrap();
    //     // println!("{:?}", diff);
    //     // TODO: Handle errors, "you have already submitted this code", actually wait for
    //     // submission result, 
    //     // Contest submit page.
    //     // println!("{:?}", utils::get_problem_details_cwd());
    //     // let problem_info = cf::ProblemInfo::from_path(std::env::current_dir().unwrap(), root_dir);
    //     
    //     // let path = problem_info.get_path();
    //     // println!("{:?}", path);
    //     // return Ok(());

    //     let url = "https://codeforces.com/contest/1976/submit";

    //     // Get csrf by doing initial response
    //     let csrf_token = self.get_csrf(url)?;

    //     let post_body = serde_qs::to_string(&PostBody::Submit {
    //         csrf_token,
    //         ftaa: self.session.ftaa.clone(),
    //         bfaa: self.session.bfaa.clone(),
    //         tta: self.session.tta.clone(),
    //         contest_id: String::from("1976"),
    //         problem_index: String::from("A"),
    //         language_id: String::from("70"),
    //         source: String::from("print('testing')"),
    //         tab_size: 4,
    //         source_file: String::from(""),
    //     }).with_context(|| "Failed to create query string for post request.")?;

    //     // Make a POST request to the url, which redirects us to the home page.
    //     let response = self.client.post(url)
    //         .header("Content-Type", "application/x-www-form-urlencoded")
    //         .body(post_body).send()
    //         .with_context(|| format!("Failed to make request to {}", url))?.text()
    //         .with_context(|| format!("Failed to get response from {}", url))?;

    //     Ok(())
    // }

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
        let handle = Self::find_handle(&html);

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
