use scraper::{Html, Selector};
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;

/// Finds the CSRF token in an HTML document served by codeforces.
pub fn csrf(html: &Html) -> Option<String> {
    // Technically there should only be one csrf token but i'm doing this to be safe lol
    let csrf_selector = Selector::parse("span.csrf-token").unwrap();
    let csrf_tokens = html.select(&csrf_selector);
    for csrf_token in csrf_tokens {
        if let Some(s) = csrf_token.value().attr("data-csrf") {
            return Some(s.to_string());
        }
    }
    None
}

/// Finds the handle of a logged-in user in the top-right 
/// corner of an HTML document served by codeforces.
pub fn handle(html: &Html) -> Option<String> {
    let lang_chooser_selector = Selector::parse("div.lang-chooser").unwrap();
    let a_selector = Selector::parse("a").unwrap();
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

/// Finds the samples on the problems page of a contest
/// for example, www.codeforces.com/contest/123/problems
pub fn sample_tests(html: &Html) -> Result<HashMap<String, Vec<(String, String)>>> {
    // TODO: It might be better to make a struct for the return value,
    // but for now this will work.
    let pdiv_selector = Selector::parse("div.problemindexholder").unwrap();
    let indiv_selector = Selector::parse("div.input").unwrap();
    let outdiv_selector = Selector::parse("div.output").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();

    let mut result = HashMap::new();

    for problemdiv in html.select(&pdiv_selector) {
        let problem_index = problemdiv.value().attr("problemindex")
            .with_context(|| "Failed to parse samples from HTML")?.to_string();

        // Flattening here might not be the best way of doing this...
        // Might break in the future.
        // TBH this entire project will definitely break at some point anyways.
        
        let indivs = problemdiv.select(&indiv_selector).map(|indiv|
            indiv.select(&pre_selector).next()).flatten().map(|pre| 
                pre.text().map(|s| s.to_string()).join("\n"));

        let outdivs = problemdiv.select(&outdiv_selector).map(|outdiv|
            outdiv.select(&pre_selector).next()).flatten().map(|pre| 
                pre.text().map(|s| s.to_string()).join("\n"));

        result.insert(problem_index, indivs.zip(outdivs).collect());
    }

    Ok(result)
}
