use dialoguer::Input;
use url::Url;
use robotstxt::DefaultMatcher;
use scraper::{Html, Selector};
use rusqlite::Connection;
use tokio::signal;
use std::time::Duration;
use std::collections::VecDeque;
use reqwest::Client;

pub async fn crawl(history: &mut Vec<String>, conn: &Connection) {
    let input: String = Input::new()
        .with_prompt("Enter the URL to crawl")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    history.push(input.clone());

    println!("Crawling URL: {}", input);

    let input: Url = Url::parse(&input).expect("Failed to parse URL");
    // crawling(&input, &conn);
    
    let mut links: VecDeque<Url> = VecDeque::new();
    links.push_back(input.clone()); 

    println!("Crawling started🚀 CRL+C TO STOP🛑");

    tokio::select! {
        _ = async {
            while let Some(input) = links.pop_front() {
                if url_exists(&conn, input.as_str()) {
                    conn.execute(
                        "UPDATE crawl SET counter = counter + 1 WHERE url = ?1",
                        &[input.as_str()],
                    ).expect("Failed to update URL counter in database");
                    // scraping_web(&input, &conn, &mut links);
                    // Dont know if scrape if it was scraped
                    continue;
                } 
                
                let mut matcher = DefaultMatcher::default();
                
                // If link is broken just continue
                let mut robots_txt = match get_robot(&input).await {
                    Ok(robots) => robots,
                    Err(e) => {
                        println!("Failed to get robots.txt for {}: {}", input, e);
                        continue;
                    }
                };
                
                // println!("Robot.txt: {:#?}", robots_txt);  
                
                if !matcher.one_agent_allowed_by_robots(&robots_txt, "my_crawler", input.as_str()) {
                    println!("Crawling is NOT allowed for the URL: {}", input);
                    continue;
                }
                println!("Crawling is allowed for the URL: {}", input);
                
                // if !url_exists(&conn, &input.to_string()) {
                conn.execute(
                    "INSERT INTO crawl (url, title) 
                    VALUES (?1, ?2)",
                    &[input.as_str(), &"".to_string()],
                ).expect("Failed to insert URL into database");
        
                scraping_web(&input, &conn, &mut links).await;
                    // } else {
                        //     conn.execute(
                            //         "UPDATE crawl SET counter = counter + 1 WHERE url = ?1",
                            //         &[&input.to_string()],
                //     ).expect("Failed to update URL counter in database");
        
                // }
            }
        } => {
            println!("Crawling ended.");
        }    
        _ = signal::ctrl_c() => {
            println!("Crawling stopped by user.");
        }
    }



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


async fn get_robot(parsed_url: &Url) -> Result<String, Box<dyn std::error::Error>> {
    let robots_url = parsed_url.join("/robots.txt")?;

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .expect("Failed to build HTTP client");

    let response = client.get(robots_url.as_str()).send().await?;

    let content = response.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());

    Ok(content)

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


// Change to add links in a qeue
async fn scraping_web(url: &Url, conn: &Connection, urls: &mut VecDeque<Url>) {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .expect("Failed to build HTTP client");

    let response = match client.get(url.as_str()).send().await {
        Ok(res) => {
            if res.status().is_success() {
                res
            } else {
                println!("Failed to crawl URL: HTTP {}", res.status());
                return;
            }
        }
        Err(err) => {
            println!("Error crawling URL: {}", err);
            return;
        }
    };

    let body = response.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
    // println!("Crawled content:\n{}", body);

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    // let mut links: Vec<String> = Vec::new();
    // println!("========= Links found on the page =========");

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let href = href.trim();
            // println!("{:?}", href);
            if href.starts_with("#") || href.is_empty() {
                continue; // Skip anchors
            }

            match url.join(href) {
                Ok(mut absolute_url) => {
                    absolute_url.set_fragment(None);

                    if (absolute_url.scheme() != "http" && absolute_url.scheme() != "https") {
                        continue; // Skip non-http/https URLs
                    }

                    let final_url_str = absolute_url.as_str();
                    urls.push_back(absolute_url.clone());
                    println!("{}", final_url_str);

                    // Maybe just add it and check with db
                    // let mut stmt: i32 = conn.query_row("SELECT COUNT(*) FROM crawl WHERE url = ?1", [final_url_str.clone()], |row| row.get(0)).expect("Failed to execute query");

                    // // url doesnt exists in the db
                    // if stmt == 0 {
                    //     urls.push_back(Url::parse(&final_url_str).expect("Failed to parse URL"));
                    //     conn.execute(
                    //         "INSERT INTO crawl (url, title) VALUES (?1, ?2)",
                    //         &[&final_url_str, &"".to_string()],
                    //     ).expect("Failed to insert URL into database");
                    // // url exists in the db
                    // } else {
                    //     conn.execute(
                    //         "UPDATE crawl SET counter = counter + 1 WHERE url = ?1",
                    //         &[&final_url_str],
                    //     ).expect("Failed to update URL counter");
                    // }
                    // println!("{}", final_url_str);
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
}
 
// async fn crawling(url: &Url, conn: &Connection) {
//     let mut robots_txt = get_robot(&url).expect("Failed to get robots.txt URL");


//     // println!("Robot.txt: {:#?}", robots_txt);  
 
//     if robots_txt.can_fetch("my_crawler", &url) {
//         println!("Crawling is allowed for the URL: {}", url);
//     } else {
//         println!("Crawling is NOT allowed for the URL: {}", url);
//     }

//     let links: Vec<String> = scraping_web(&url, &conn).expect("Failed to scrape the web page");

//     // Add tokio to stop with CTRL+C
//     for link in links {
//         crawling(&Url::parse(&link).expect("Failed to parse URL"), conn).await;
//     }
// }

fn url_exists(conn: &Connection, url: &str) -> bool {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM crawl WHERE url = ?1").expect("Failed to prepare statement");
    let count: i64 = stmt.query_row([url], |row| row.get(0)).expect("Failed to execute query");
    count > 0
}