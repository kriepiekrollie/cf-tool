use crate::cli::contest::ContestArgs;//, ContestInfo};

pub fn parse_samples(args: &ContestArgs) {
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
