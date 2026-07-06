use dialoguer::Input;
use url::Url;
use robotparser::parser::parse_robots_txt;

pub fn crawl(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("Enter the URL to crawl")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    history.push(input.clone());

    let mut domain = get_robot(&input).expect("Failed to get robots.txt URL");
    println!("Crawling URL: {domain}");

    // get the robots.txt content
    let robot_txt = match reqwest::blocking::get(&domain) {
        Ok(res) => {
            if res.status().is_success() {
                let robot_txt = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
                // println!("robots.txt content:\n{}", robot_txt);
                history.push(format!("robots.txt content:\n{}", robot_txt));
                robot_txt
            } else {
                // println!("Failed to fetch robots.txt: HTTP {}", res.status());
                history.push(format!("Failed to fetch robots.txt: HTTP {}", res.status()));
                "Failed to fetch robots.txt".to_string()
            }
        }

        Err(err) => {
            let error_message = format!("Error fetching robots.txt: {}", err);
            // println!("{}", error_message);
            history.push(error_message.clone());
            error_message
        }
    };

    println!("{}", robot_txt);

    // it might go into other function
    let user_agent = "HomeMadeCrawler"; // Replace with your crawler's user-agent string

    let url_base = Url::parse(&domain).unwrap_or_else(|_| Url::parse("https://example.com").unwrap());
    let origin = robotparser::component::Origin::from(url_base);    

    let robots_parsed = parse_robots_txt(origin, &robot_txt);

    // 2. Ahora usamos el objeto resultante para comprobar las rutas
    let mi_user_agent = "MiBotCrawler";
    let ruta_a_visitar = "/ruta-interesante"; // o la URL completa, según pida tu implementación

    if robots_parsed.can_fetch(mi_user_agent, ruta_a_visitar) {
        println!("✅ El robots.txt nos permite entrar a {}", ruta_a_visitar);
        // Aquí continúas con tu reqwest...
    } else {
        println!("❌ Acceso denegado por robots.txt para {}", ruta_a_visitar);
    }

    // let parser = match RobotsTxt::from_str(&robot_txt) {
    //     Ok(parser) => {
    //         // 2. Comprobamos si tu bot tiene permiso para entrar a una ruta específica
    //         // El método .can_fetch pide: (Nombre de tu bot, la ruta interna que quieres visitar)
    //         if parser.can_fetch("MiBotCrawler", &input) {
    //             println!("✅ OK from robots.txt!");
    //             // Aquí puedes meter tu lógica para descargar la página con reqwest
    //         } else {
    //             println!("❌ Prohibited by robots.txt");
    //         }
    //         parser
    //     }

    //     Err(e) => {
    //         println!("Structure not processed by robots.txt: {:?}", e);
    //         return;
    //     }
    // };

    // if parser.can_fetch(user_agent, &input) {
    //     println!("Crawling allowed for URL: {}", input);
    //     history.push(format!("Crawling allowed for URL: {}", input));
    // } else {
    //     println!("Crawling disallowed for URL: {}", input);
    //     history.push(format!("Crawling disallowed for URL: {}", input));
    //     return;
    // }

    let response = match reqwest::blocking::get(&input) {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
                println!("Crawled content:\n{}", body);
                history.push(format!("Crawled content:\n{}", body));
                body
            } else {
                let error_message = format!("Failed to crawl URL: HTTP {}", res.status());
                println!("{}", error_message);
                history.push(error_message);
                return;
            }
        }

        Err(err) => {
            let error_message = format!("Error crawling URL: {}", err);
            println!("{}", error_message);
            history.push(error_message);
            return;
        }

    };
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