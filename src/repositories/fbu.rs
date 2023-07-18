// Crates
use log::info;
// Repo Modules
use crate::models::fbu::FbuResponse;

// Local Functions



pub fn format_fbu(response: (FbuResponse,FbuResponse,FbuResponse)) -> String {
    info!("Formatting FBU");

    let allergens = &response.0.article.allergens;

    let response_fresh = response.0.to_owned();
    let response_street = response.1;
    let response_flow = response.2;

    let lunch_menu_fresh = get_lunch_menu(response_fresh);
    let lunch_menu_street = get_lunch_menu(response_street);
    let lunch_menu_flow = get_lunch_menu(response_flow);

    let message_fresh = format_message(lunch_menu_fresh);
    let message_street = format_message(lunch_menu_street);
    let message_flow = format_message(lunch_menu_flow);

    let message = message_fresh + &message_street + &message_flow + "\n\n" + allergens + "\n";
    info!("Returning formatted FBU response");
    message
}


fn get_lunch_menu(response: FbuResponse) -> Vec<String> {
    info!("Processing the lunch menu");

    let mut lunch_menu: Vec<String> = Vec::new();

    
  let canteen = response.category.name;
  let menu_description = response.article.description;

  let search_string = "\r\n";
  let dot_pattern = "..";
  let hyphen_pattern = "--";

  

  lunch_menu.push(canteen);

  let splits = menu_description.split(search_string);
  for split in splits {
      if split != ""  || split.contains(hyphen_pattern) {
          if split.contains(dot_pattern) || split.contains(hyphen_pattern) {
              break;
          }
          else {
              lunch_menu.push(split.to_string());
          }
      }
  }
  info!("Returning lunch menu");



    lunch_menu
}


fn format_message (lunch_menu: Vec<String>) -> String {
    info!("Formatting the message");
    let mut message = String::new();
    let mut counter = 0;

    for item in lunch_menu {
        if counter == 0 {
            message = "# ".to_string() + &item + "\n";
        }
        if counter == 1 {
            message = message + &item + "\n";
        }
        if counter > 1 {
            message = message + "- " + &item + "\n";    
        }

        counter = counter + 1;
    }
    info!("Returning formatted message");
    message
}