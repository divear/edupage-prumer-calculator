use colored::Colorize;
use dotenvy::dotenv;
// mod prev;
// use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::{env, error::Error, /* thread,*/ time::Duration};

const WAIT_LIMIT: u64 = 15;

fn process_two(znamka: &str) -> Option<f32> {
    // 2- etc.
    println!("{} passed into process_two", znamka);
    let returned = znamka.chars().nth(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    println!("{:?}", returned_num);
    Some(returned_num + 0.25)
}
fn process_longer(znamka: &str) -> Option<f32> {
    // 9 / 15 = 60% → 3 etc.
    println!("{} passed into process_longer", znamka);

    let returned = znamka.chars().nth_back(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    println!("{:?}", returned_num);
    Some(returned_num)
}
fn process_percent(znamka: &str) -> Option<f32> {
    // 100% etc.
    println!("{} passed into process_percent", znamka);
    let znamka: f32 = znamka.replace("%", "").parse().expect("a proper float");
    println!("{}", znamka);
    Some(znamka)
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    dotenv().expect(".env file not found");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    println!("signing in");
    tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
    tab.wait_for_element("input#home_Login_2e1")?.click()?;
    tab.type_str(&username)?;
    tab.wait_for_element("input#home_Login_2e2")?.click()?;
    tab.type_str(&password)?.press_key("Enter")?;

    println!("znamky page");
    tab.navigate_to("https://sspbrno.edupage.org/znamky")?
        .wait_for_element_with_custom_timeout(
            "#edubarStartButton",
            Duration::from_secs(WAIT_LIMIT),
        )?; //without this it doesn't work; investigate;

    println!("start");
    let inner_text_content = tab
        .wait_for_element_with_custom_timeout(".znZnamka", Duration::from_secs(WAIT_LIMIT))?
        .get_inner_text()?;
    println!("{}", inner_text_content);
    println!("end");
    // let test_button = tab
    //     .find_elements("znznamka")?
    //     .capture_screenshot(page::capturescreenshotformatoption::png)?;
    // std::fs::write("button.jpeg", test_button)?;
    let mut znamky_all: Vec<f32> = Vec::new();
    let containing_element = tab.find_elements(".znZnamka")?;
    for i in &containing_element {
        println!("{:?}", i);
        let new_znamka = i.get_inner_text();

        match new_znamka {
            Ok(new_znamka) => {
                println!("new znamka: {:?}", new_znamka);
                match new_znamka.parse::<f32>() {
                    Ok(znamka_int) => {
                        println!("Parsed number: {}", znamka_int.to_string().green());
                        znamky_all.push(znamka_int);
                    }
                    Err(_) => {
                        println!("'{}' není v normálním formátu", new_znamka.yellow());
                        if new_znamka.chars().nth_back(0) == Some('%') {
                            process_percent(&new_znamka);
                        } else {
                            let extracted_znamka = match new_znamka.len() {
                                1 => {
                                    println!("+/-/o/S se nevztahuje na prumer");
                                    None
                                }
                                2 => process_two(&new_znamka),
                                3.. => process_longer(&new_znamka),
                                _ => {
                                    println!("the length of grade is 0 or negative!");
                                    None
                                }
                            };
                            if extracted_znamka.is_none() {
                                println!("this grade was not counted into the average")
                            } else {
                                // let parsed_znamka: usize =
                                //     extracted_znamka.unwrap_or(' ').to_string().parse()?;
                                if extracted_znamka.is_some() {
                                    znamky_all.push(
                                        extracted_znamka.expect("adding a working grade failed"),
                                    )
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to get inner text: {:?}", e);
                // Handle the error case where `get_inner_text` failed.
            }
        }
        // znamky_all.push(znamka_int);
    }
    // let inner_divs = containing_element.find_elements(".znznamka")?;
    println!("{:?}", znamky_all);

    // let curr_prumer = tab.wait_for_element(".expandImg")?.get_inner_text()?;
    // println!("{}", curr_prumer);

    Ok(())
}
