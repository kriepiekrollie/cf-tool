use crate::cf::SubmissionInfo;
use scraper::{Html, Selector, ElementRef};
use anyhow::{Context, Result, anyhow, bail};
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

/// Find the problem index from the list of options in the select.
/// Basically a check to see if the problem actually exists.
pub fn problem_index(html: &Html, problem_id: &str) -> Option<String> {
    let pselect_selector = Selector::parse("select[name=\"submittedProblemIndex\"]").unwrap();
    let option_selector = Selector::parse("option").unwrap();
    for select in html.select(&pselect_selector) {
        for option in select.select(&option_selector) {
            if let Some(value) = option.value().attr("value") {
                if value.to_lowercase() == problem_id.to_lowercase() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn parse_submission_tr(row: &ElementRef<'_>) -> Result<SubmissionInfo> {
    let td_selector = Selector::parse("td").unwrap();
    let mut iter = row.select(&td_selector);
    let id = iter.next().with_context(|| "Couldn't find submission ID.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let when = iter.next().with_context(|| "Couldn't find submission time.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let who = iter.next().with_context(|| "Couldn't find submission author.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let problem = iter.next().with_context(|| "Couldn't find submission problem.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let lang = iter.next().with_context(|| "Couldn't find submission language.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let verdict = iter.next().with_context(|| "Couldn't find submission verdict.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let time = iter.next().with_context(|| "Couldn't find result time.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    let memory = iter.next().with_context(|| "Couldn't find result memory.")?
        .text().map(|s| s.to_string()).collect::<String>().trim().to_string();
    Ok(SubmissionInfo {
        id,
        when,
        who, 
        problem, 
        lang, 
        verdict, 
        time, 
        memory,
    })
}

pub fn latest_submission_info(html: &Html) -> Result<SubmissionInfo> {
    let table_selector = Selector::parse("table.status-frame-datatable").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let mut cur_latest = "0";
    let mut submission_tr = None;
    for table in html.select(&table_selector) {
        for tr in table.select(&tr_selector) {
            if let Some(sid) = tr.value().attr("data-submission-id") {
                if cur_latest < sid {
                    cur_latest = sid;
                    submission_tr = Some(tr);
                }
            }
        }
    }
    let submission_tr = submission_tr.with_context(|| "Failed to find submission in table.")?;
    Ok(parse_submission_tr(&submission_tr)
        .with_context(|| "Failed to parse submission information from table.")?)
}

pub fn submission_info(html: &Html, submission_id: &String) -> Result<SubmissionInfo> {
    let table_selector = Selector::parse("table.status-frame-datatable").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    for table in html.select(&table_selector) {
        for tr in table.select(&tr_selector) {
            if let Some(sid) = tr.value().attr("data-submission-id") {
                if sid.to_string() == *submission_id {
                    return Ok(parse_submission_tr(&tr).with_context(|| 
                        "Failed to parse submission information from table.")?)
                }
            }
        }
    }
    bail!("Failed to find submission in table.")
}
