use colored::Colorize;
use dotenvy::dotenv;
// mod prev;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::{
    collections::HashSet, env, error::Error, fs, io, io::prelude::*,
    /* thread,*/ time::Duration,
};

const WAIT_LIMIT: u64 = 15;
#[derive(Debug)]
struct ZnamkaStruct {
    predmet: String,
    nazev: String,
    znamka: f32,
    vaha: f32,
}
// struct PredmetStruct {
//     predmet: String,
//     znamky: Vec<f32>,
// }

// the individual processing functions
fn process_two(znamka: &str) -> Option<f32> {
    // 2- etc.
    println!("{} passed into process_two", znamka);
    let returned = znamka.chars().nth(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    println!("{:?}", returned_num);
    Some(returned_num + 0.5)
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
    dotenv().expect(".env file not found - follow instructions in the README");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    loop {
        println!("signing in");
        tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
        tab.wait_for_element("input#home_Login_2e1")?.click()?;
        tab.type_str(&username)?;
        tab.wait_for_element("input#home_Login_2e2")?.click()?;
        tab.type_str(&password)?.press_key("Enter")?;

        println!("znamky page");

        tab.navigate_to("https://sspbrno.edupage.org/znamky/?eqa=d2hhdD1zdHVkZW50dmlld2VyJnBvaGxhZD1wb2RsYURhdHVtdSZ6bmFta3lfeWVhcmlkPTIwMjMmem5hbWt5X3llYXJpZF9ucz0xJm5hZG9iZG9iaWU9UDImcm9rb2Jkb2JpZT0yMDIzJTNBJTNBUDImZG9ScT0xJndoYXQ9c3R1ZGVudHZpZXdlciZ1cGRhdGVMYXN0Vmlldz0w")?;
        println!("navigated to znamky page");
        match tab.wait_for_element_with_custom_timeout(
            "#edubarStartButton",
            Duration::from_secs(WAIT_LIMIT),
        ) {
            Ok(d) => {
                Some(d);
                break;
            }
            Err(_) => {
                println!("could not find, trying again");
            }
        };
    }

    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
    // Save the screenshot to disc
    fs::write("./assets/screenshot.png", jpeg_data)?;

    println!("start");
    let prvni_znamka = tab
        .wait_for_element_with_custom_timeout(".znZnamka", Duration::from_secs(WAIT_LIMIT))?
        .get_inner_text()?;
    let _prvni_predmet = tab
        .wait_for_element_with_custom_timeout(
            ".app-list-item-main div b",
            Duration::from_secs(WAIT_LIMIT),
        )?
        .get_inner_text()?;
    println!("{}", prvni_znamka);
    println!("{}", _prvni_predmet);
    println!("end");

    // let znamky_all: Vec<f32> = Vec::new();
    let mut everything_vec: Vec<ZnamkaStruct> = vec![];
    // let predmety_all: Vec<String> = Vec::new();
    // let znamky_elements = tab.find_elements(".znZnamka")?;
    // let predmety_elements = tab.find_elements(".app-list-item-main div:first-of-type b")?;
    let everything = tab.find_elements(".app-list-item-main")?;
    // println!("{:?}", &everything.first().get_inner_text());

    for i in everything {
        let inner_text = i.get_inner_text()?;
        let all: Vec<&str> = inner_text.lines().collect();

        println!("{:?}", all);
        let new_znamka = all[2];
        if all[0] == "Chování" || all[1] == "Vysvědčení" {
            continue;
        }

        // match new_znamka {
        //     Ok(new_znamka) => {
        //         println!("new znamka: {:?}", new_znamka);
        let created_znamka = match new_znamka.parse::<f32>() {
            Ok(znamka_int) => {
                println!("Parsed number: {}", znamka_int.to_string().green());
                Ok(znamka_int)
            }
            Err(_) => {
                println!("'{}' není v normálním formátu", new_znamka.yellow());
                if new_znamka.chars().nth_back(0) == Some('%') {
                    let extracted_znamka = process_percent(&new_znamka);
                    if extracted_znamka.is_some() {
                        Ok(extracted_znamka.expect("adding a working grade failed"))
                    } else {
                        Err("extracted_znamka doesnt exist")
                    }
                } else {
                    println!("new_znamka: {:?}", &new_znamka);
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
                        Err("this grade was not counted into the average")
                    } else {
                        println!("{:?}", extracted_znamka);
                        Ok(extracted_znamka.expect("adding a working grade failed"))
                    }
                }
            }
        };
        match created_znamka {
            Ok(created_znamka) => {
                if let Some(start) = all[1].find('∙') {
                    if let Some(end) = all[1].find('×') {
                        if start < end {
                            let start_pos = start + '∙'.len_utf8();
                            let result = &all[1][start_pos..end].trim();
                            let vaha = result.parse().unwrap();
                            println!("vaha: {}", vaha);
                            let new_znamka_instance = ZnamkaStruct {
                                predmet: all[0].to_string(),
                                nazev: all[1].to_string(),
                                znamka: created_znamka,
                                vaha,
                            };
                            println!("vaha saved: {}", new_znamka_instance.vaha);
                            println!("znamka saved: {}", new_znamka_instance.znamka);
                            everything_vec.push(new_znamka_instance);
                        } else {
                            println!("The '×' character comes before the '∙' character.");
                        }
                    } else {
                        println!("The '×' character was not found.");
                    }
                } else {
                    println!("The '∙' character was not found.");
                }
            }
            Err(_) => {
                println!("error: the created_znamka isn't valid");
            }
        };
        println!("{:?}", created_znamka);
    }

    let mut set_existujicich_predmetu = HashSet::new();
    for i in &everything_vec {
        // println!("{:?}", &i);
        set_existujicich_predmetu.insert(&i.predmet);
    }
    for (i, p) in set_existujicich_predmetu.clone().into_iter().enumerate() {
        println!("{}) - {}", i, p);
        // for j in &everything_vec {
        //     if (j == i) {}
        // }
    }
    for _ in 1..10 {
        print!("Pro který předmět chcete vypočítat průměr? ");
        io::stdout().flush().unwrap(); // Ensures the prompt is displayed before waiting for input
        let predmet_pick_index: usize = io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .parse()?;
        // random order, because HashSet
        let predmet_pick = Vec::from_iter(&set_existujicich_predmetu)[predmet_pick_index];
        println!("\nVybrali jste: {}", predmet_pick);
        let mut x: Vec<f32> = vec![];
        let mut picked_predmet_znamky: Vec<f32> = vec![];
        let mut picked_predmet_vahy: Vec<f32> = vec![];
        for i in &everything_vec {
            if i.predmet == **predmet_pick {
                x.push(i.znamka);
                picked_predmet_znamky.push(i.znamka * i.vaha);
                picked_predmet_vahy.push(i.vaha);
            }
        }
        println!("{:?}", x);
        println!("{:?}", picked_predmet_vahy);
        println!("{:?}", picked_predmet_znamky);
        println!(
            "Váš stávající průměr: {}",
            picked_predmet_znamky.clone().into_iter().sum::<f32>()
                / picked_predmet_vahy.clone().into_iter().sum::<f32>()
        );
    }
    Ok(())
}
