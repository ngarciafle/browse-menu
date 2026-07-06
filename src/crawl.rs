use dialoguer::Input;
use url::Url;
use robotparser::parser::parse_robots_txt;
use robotparser::service::RobotsTxtService;
use robotparser::model::RobotsTxt;

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