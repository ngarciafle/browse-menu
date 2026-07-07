use dialoguer::Input;
use url::Url;
use robotparser::parser::parse_robots_txt;
use robotparser::service::RobotsTxtService;
use robotparser::model::RobotsTxt;
use scraper::{Html, Selector};

pub fn crawl(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("Enter the URL to crawl")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    history.push(input.clone());

    println!("Crawling URL: {}", input);

    let input: Url = Url::parse(&input).expect("Failed to parse URL");

    let mut robots_txt = get_robot(&input).expect("Failed to get robots.txt URL");


    println!("Robot.txt: {:#?}", robots_txt);  
 
    if robots_txt.can_fetch("my_crawler", &input) {
        println!("Crawling is allowed for the URL: {}", input);
    } else {
        println!("Crawling is NOT allowed for the URL: {}", input);
    }

    let links: Vec<String> = scraping_web(&input).expect("Failed to scrape the web page");

    // Petition as a EXAMPLE
    // let response = match reqwest::blocking::get(&input) {
    //     Ok(res) => {
    //         if res.status().is_success() {
    //             let body = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
    //             println!("Crawled content:\n{}", body);
    //             history.push(format!("Crawled content:\n{}", body));
    //             body
    //         } else {
    //             let error_message = format!("Failed to crawl URL: HTTP {}", res.status());
    //             println!("{}", error_message);
    //             history.push(error_message);
    //             return;
    //         }
    //     }
        
    //     Err(err) => {
    //         let error_message = format!("Error crawling URL: {}", err);
    //         println!("{}", error_message);
    //         history.push(error_message);
    //         return;
    //     }
        
    // };
}


fn get_robot(parsed_url: &Url) -> Result<RobotsTxt, url::ParseError> {
    // http or https
    let scheme = parsed_url.scheme();
    
    let domain = match parsed_url.host_str() {
        Some(domain) => domain,
        None => return Err(url::ParseError::EmptyHost),
    };
    
    let robots_url = if let Some(port) = parsed_url.port() {
        format!("{}://{}:{}/robots.txt", scheme, domain, port)
    } else {
        format!("{}://{}/robots.txt", scheme, domain)
    };
    let robots_url = Url::parse(&robots_url).expect("Failed to parse robots.txt URL");

    let robots_txt = parse_robots_txt(robots_url.origin(), "User-Agent: *\nDisallow: /search\n");
    assert!(robots_txt.get_warnings().is_empty(), "Failed to parse robots.txt: {:?}", robots_txt.get_warnings());
    let robots_txt = robots_txt.get_result();

    return Ok(robots_txt);

    // NOT NECESSARY WITH ACTUAL IMPLEMENTATION 
    // get the robots.txt content
    // i dont know if its needed
    // let robot_txt = match reqwest::blocking::get(&robots_url) {
    //     Ok(res) => {
    //         if res.status().is_success() {
    //             let robot_txt = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
    //             // println!("robots.txt content:\n{}", robot_txt);
    //             history.push(format!("robots.txt content:\n{}", robot_txt));
    //             robot_txt
    //         } else {
    //             // println!("Failed to fetch robots.txt: HTTP {}", res.status());
    //             history.push(format!("Failed to fetch robots.txt: HTTP {}", res.status()));
    //             "Failed to fetch robots.txt".to_string()
    //         }
    //     }
    
    //     Err(err) => {
    //         let error_message = format!("Error fetching robots.txt: {}", err);
    //         // println!("{}", error_message);
    //         history.push(error_message.clone());
    //         error_message
    //     }
    // };   
}

fn scraping_web(url: &Url) -> Result<Vec<String>, String> {
    let response = reqwest::blocking::get(url.clone()).expect("Failed to send request");
    if !response.status().is_success() {
        let error_message = format!("Failed to crawl URL: HTTP {}", response.status());
        println!("{}", error_message);
        return Err("The request was not successful".to_string());
    }

    let body = response.text().unwrap_or_else(|_| "Failed to read response body".to_string());
    println!("Crawled content:\n{}", body);

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    let mut links: Vec<String> = Vec::new();
    println!("========= Links found on the page =========");

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.starts_with("#") {
                continue; // Skip anchors
            }

            match url.join(href) {
                Ok(mut absolute_url) => {
                    absolute_url.set_fragment(None);

                    let final_url_str = absolute_url.to_string();

                    if !links.contains(&final_url_str) {
                        links.push(final_url_str.clone());
                        println!("{}", final_url_str);
                    }
                }
                Err(_) => {
                    continue;
                }
            }

            // Relative URLs
            // if href.starts_with("/") {
            //     let base_url = url.clone();
            //     let absolute_url = base_url.join(href).expect("Failed to join URLs");
            //     links.push(absolute_url.to_string());
            //     println!("{}", absolute_url);
            //     continue;
            // }

            // if href.starts_with("http://") || href.starts_with("https://") {
            //     links.push(href.to_string());
            //     println!("{}", href);
            //     continue;
            // }
        }
    }

    Ok(links)
}