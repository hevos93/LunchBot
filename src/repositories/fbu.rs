// Crates
use log::info;
// Repo Modules
use crate::models::fbu::{FbuResponse, FbuJSON, Fresh, Street, Flow};

// Local Functions



pub fn format_fbu(response: (FbuResponse,FbuResponse,FbuResponse), day: &String) -> String {
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

    let formatted_day = format_day(day);

    let message = formatted_day + &message_fresh + &message_street + &message_flow + "\n\n" + allergens + "\n";
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
            message = "### ".to_string() + &item + "\n";
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

fn format_day(day: &String) -> String {
    info!("Formatting day");
    let formatted_day = match day.as_str() {
        "monday" => "## Mandag\n",
        "tuesday" => "## Tirsdag\n",
        "wednesday" => "## Onsdag\n",
        "thursday" => "## Torsdag\n",
        "friday" => "## Fredag\n",
        _ => ""
    };
    info!("Returning formatted day");
    formatted_day.to_string()
}

pub fn format_fbu_json(response: (FbuResponse,FbuResponse,FbuResponse), day: &String) -> FbuJSON {
    info!("Formatting FBU JSON");

    let allergens = &response.0.article.allergens;
    let day = day;

    let response_fresh = response.0.to_owned();
    let response_street = response.1;
    let response_flow = response.2;

    let lunch_menu_fresh = get_lunch_menu(response_fresh);
    let lunch_menu_street = get_lunch_menu(response_street);
    let lunch_menu_flow = get_lunch_menu(response_flow);

    let json_response = format_json(lunch_menu_fresh, lunch_menu_street, lunch_menu_flow, allergens, day);
    info!("Returning JSON");
    json_response
}

fn format_json(lunch_menu_fresh: Vec<String>, lunch_menu_street: Vec<String>, lunch_menu_flow: Vec<String>, allergens: &String, day: &String) -> FbuJSON {
    info!("Formatting to JSON");
    let fresh_obj = get_obj_json(lunch_menu_fresh);
    let street_obj = get_obj_json(lunch_menu_street);
    let flow_obj = get_obj_json(lunch_menu_flow);

    let fresh: Fresh = Fresh {
        name: fresh_obj.0,
        info: fresh_obj.1,
        menu: fresh_obj.2
    };

    let street: Street = Street {
        name: street_obj.0,
        info: street_obj.1,
        menu: street_obj.2
    };

    let flow: Flow = Flow {
        name: flow_obj.0,
        info: flow_obj.1,
        menu: flow_obj.2
    };


    let response: FbuJSON = FbuJSON { 
        fresh: fresh, 
        street: street,
        flow: flow,
        allergens: allergens.to_string(),
        day: day.to_string() 
    };
    info!("Returning formatted JSON");
    response
}

fn get_obj_json(lunch_menu: Vec<String>) -> (String, String, String) {
    info!("Extracting object");
    let mut response: (String, String, String) = ("Name".to_string(), "Info".to_string(), "Menu".to_string());
    let mut menu = String::new();
    let mut counter = 0;

    for item in lunch_menu {
        if counter == 0 {
            response.0 = item.clone();
        }
        if counter == 1 {
            response.1 = item.clone();
        }
        if counter > 1 {
            menu = menu + "- " + &item + "\n";    
        }

        counter = counter + 1;
    };
    response.2 = menu;

    info!("Returning object");
    response
}