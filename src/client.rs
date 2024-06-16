use crate::cli::ContestArgs;
use std::path::{Path, PathBuf};
use crate::config::LoginDetails;
use colored::Colorize;
use serde::{Serialize, Deserialize};

pub struct Client {
    client: reqwest::blocking::Client,
    cookie_store: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
}

impl Client {
    pub fn new() -> Self {
        let cookie_store = reqwest_cookie_store::CookieStore::new(None);
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        let client = reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build()
            .unwrap();
        Self {
            client,
            cookie_store: cookie_store,
        }
    }
    pub fn load(path: &PathBuf) -> Self {
        let cookie_store = {
            let file = std::fs::File::open(path)
                .map(std::io::BufReader::new)
                .unwrap();
            reqwest_cookie_store::CookieStore::load_json(file).unwrap()
        };
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        let client = reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build()
            .unwrap();
        Self {
            client,
            cookie_store: cookie_store,
        }
    }
    pub fn load_or_new(path: &PathBuf) -> Self {
        if path.exists() {
            Self::load(path)
        } else {
            Self::new()
        }
    }
    pub fn save(&self, path: &PathBuf) {
        let mut writer = std::fs::File::create(path)
            .map(std::io::BufWriter::new)
            .unwrap();
        let store = self.cookie_store.lock().unwrap();
        store.save_incl_expired_and_nonpersistent_json(&mut writer).unwrap();
    }

    pub fn login(&self, details: LoginDetails) -> bool {
        // Codeforces login page.
        let url = "https://codeforces.com/enter";

        // Make a get request to the url and find the CSRF token 
        let response = self.client.get(url).send().unwrap();
        let response_text = response.text().unwrap();
        let html_body = scraper::Html::parse_document(&response_text);
        let csrf = find_csrf(&html_body).unwrap();

        // Generate these values (i have no idea what they mean)
        let ftaa = gen_ftaa();
        let bfaa = gen_bfaa();

        // Make a POST request to the url, which redirects us to the home page.
        let response = self.client.post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "csrf_token={}&action=enter&ftaa={}&bfaa={}&handleOrEmail={}&password={}&_taa=176&remember=off", 
                csrf, ftaa, bfaa, details.handle, details.password
            )).send()
            .unwrap();

        let response_text = response.text().unwrap();
        let html_body = scraper::Html::parse_document(&response_text);

        if let handle = find_handle(&html_body) {
            true
        } else {
            false
        }

    }

    pub fn parse_samples(&self, args: &ContestArgs) {
        // let contest_info = ContestInfo::new(args);
        let url = format!("https://codeforces.com/{}/{}/problems", args.get_contest_type(), args.contest_id);
        // println!("{}", std::env::current_dir().unwrap().file_name().unwrap().to_str().unwrap());
        println!("{}", url);

        // let url = "https://codeforces.com/contest/1985/problems";

        // let response = reqwest::blocking::get(url);
        // let response_text = response.unwrap().text().unwrap();

        // let html_body = scraper::Html::parse_document(&response_text);

        // let html_problem_selector = scraper::Selector::parse("div.problemindexholder").unwrap();
        // let html_problems = html_body.select(&html_problem_selector);

        // for html_problem in html_problems {
        //     if let Some(s) = html_problem.value().attr("problemindex") {
        //         println!("{}", s);
        //     }
        // }
    }

    pub fn submit_code(&self) {
        // For now just testing if we can load cookies and log in.
        self.test_logged_in();
    }

    fn test_logged_in(&self) -> bool {
        let url = "https://codeforces.com";

        // make a simple resonse to the home page.
        let response = self.client.get(url)
            .send()
            .unwrap();

        let response_text = response.text().unwrap();
        let html_body = scraper::Html::parse_document(&response_text);

        // Try to find the handle.
        let handle = find_handle(&html_body);

        if let Some(handle) = &handle {
            println!("{}{}", "You are indeed logged in as ".green().bold(), handle.green().bold());
            true
        } else {
            println!("{}", "Failed to log in.".red().bold());
            false
        }
    }
}

fn find_csrf(body: &scraper::Html) -> Option<String> {
    let csrf_selector = scraper::Selector::parse("span.csrf-token").unwrap();

    // Technically there should only be one csrf token but i'm doing this to be safe lol
    let csrf_tokens = body.select(&csrf_selector);

    for csrf_token in csrf_tokens {
        if let Some(s) = csrf_token.value().attr("data-csrf") {
            return Some(s.to_string());
        }
    }

    return None;
}

use rand::prelude::*;
fn gen_ftaa() -> String {
    let mut rng = rand::thread_rng();
    let ftaa = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(18)
        .collect::<String>()
        .to_lowercase();
	return ftaa;
}

fn gen_bfaa() -> String {
    // Idk why but this is what xalanq did lol
	return "f1b3f18c715565b589b7823cda7448ce".to_string();
}

fn find_handle(body: &scraper::Html) -> Option<String> {
    let personal_sidebar_selector = scraper::Selector::parse("div.personal-sidebar").unwrap();
    let avatar_selector = scraper::Selector::parse("div.avatar").unwrap();
    let div_selector = scraper::Selector::parse("div").unwrap();
    let a_selector = scraper::Selector::parse("a").unwrap();

    // Technically there should only be one but i'm doing this to be safe lol

    for personal_sidebar in body.select(&personal_sidebar_selector) {
        for avatar in personal_sidebar.select(&avatar_selector) {
            for div in avatar.select(&div_selector) {
                for a in div.select(&a_selector) {
                    return Some(a.inner_html());
                }
            }
        }
    }

    return None;
}
