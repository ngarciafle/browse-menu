use dialoguer::Input;
use url::Url;

pub fn crawl(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("Enter the URL to crawl")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    history.push(input.clone());

    let mut domain = get_robot(&input).expect("Failed to get robots.txt URL");
    println!("Crawling URL: {domain}");

    match reqwest::blocking::get(&domain) {
        Ok(res) => {

            if res.status().is_success() {
                let robot_txt = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
                println!("robots.txt content:\n{}", robot_txt);
                history.push(format!("robots.txt content:\n{}", robot_txt));
            } else {
                println!("Failed to fetch robots.txt: HTTP {}", res.status());
                history.push(format!("Failed to fetch robots.txt: HTTP {}", res.status()));
            }
        }
        Err(err) => {
            println!("Error fetching robots.txt: {}", err);
            history.push(format!("Error fetching robots.txt: {}", err));
        }
    } 
}


fn get_robot(url: &str) -> Result<String, url::ParseError> {
    let parsed_url = url::Url::parse(url)?;
    // http or https
    let scheme = parsed_url.scheme();
    
    let domain = match parsed_url.host_str() {
        Some(domain) => domain,
        None => return Err(url::ParseError::EmptyHost),
    };

    let robots = if let Some(port) = parsed_url.port() {
        format!("{}://{}:{}/robots.txt", scheme, domain, port)
    } else {
        format!("{}://{}/robots.txt", scheme, domain)
    };
    
    Ok(robots)
}