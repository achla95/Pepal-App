#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::collections::HashMap;
use regex::Regex;
use scraper::{Html,Selector};
use reqwest;
use serde_json::Value;
use tauri_plugin_store::PluginBuilder;
use chrono::{self, Local, NaiveTime};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn get_cookie(username: &str, password: &str) -> Result<String, String> {
    let login_info: HashMap<&str,&str> = HashMap::from([("login", username), ("pass", password)]);
    let client = reqwest::Client::builder().cookie_store(true).build().map_err(|e| e.to_string())?;
    let req = client.post("https://www.pepal.eu/include/php/ident.php").form(&login_info).send().await.map_err(|e| e.to_string())?;
    let get_cookie = req.headers().get("set-cookie").unwrap().to_str().unwrap().to_string();
    let cookie = get_cookie.split(";").collect::<Vec<&str>>()[0].to_string();
    Ok(cookie)
}
#[tauri::command]
async fn get_name(username: &str, password : &str) -> Result<String,String> {
    let mut login_info: HashMap<&str,&str> = HashMap::new();
    login_info.insert("login", username);
    login_info.insert("pass", password);
    let client = reqwest::Client::builder().cookie_store(true).build().map_err(|e| e.to_string())?;
    client.post("https://www.pepal.eu/include/php/ident.php").form(&login_info).send().await.map_err(|e| e.to_string())?;
    let req = client.get("https://www.pepal.eu/").send().await.map_err(|e| e.to_string())?;
    let body = req.text().await.map_err(|e| e.to_string())?;
    let fragment = Html::parse_fragment(&body);
    let selector = &Selector::parse("span.username").unwrap();
    let name = fragment.select(&selector).map(|x| x.inner_html()).collect::<String>(); 

    Ok(name.trim().to_string())

}
#[tauri::command]
async fn get_notes(cookie: &str) -> Result<Vec<Vec<String>>, String> { 
    let client = reqwest::Client::builder().build().map_err(|e| e.to_string())?;
    let req = client.get("https://www.pepal.eu/?my=notes").header("Cookie", cookie).send().await.map_err(|e| e.to_string())?;
    let body = req.text().await.map_err(|e| e.to_string())?;
    let parsed_html = Html::parse_fragment(&body);
    let selector = &Selector::parse("tr.note_devoir").unwrap();
    let mut notes = vec![];
    for element  in parsed_html.select(selector){
        //retirer les \n \t puis enlever tous les espaces et enfin stocker le tout dans un Vecteur
        let mut info_txt = element.text().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        info_txt.remove(1);
        info_txt.swap(0,1);
        notes.push(info_txt);
    }
    Ok(notes)
}
#[tauri::command]
async fn get_room(cookie: &str) -> Result<String,String> {
    let client = reqwest::Client::builder().build().map_err(|e| e.to_string())?;
    let req = client.get("https://www.pepal.eu/?my=edt").header("Cookie", cookie).send().await.map_err(|e| e.to_string())?;
    let body = req.text().await.map_err(|e| e.to_string())?;
    
    let parsed_html = Html::parse_fragment(&body);
    let selector = &Selector::parse("script").unwrap();
    let script_content = parsed_html.select(&selector).map(|x| x.inner_html()).collect::<String>(); 
    let mut event = script_content.split("events:").collect::<Vec<&str>>(); event.remove(0);
    let temp = event.join("");
    let mut test2 = temp.split("}]").collect::<Vec<&str>>(); test2.remove(1);
    let mut to_string = test2.join(""); to_string.push_str("}]");
    let to_html = to_string.replace("&lt;", "<").replace("&gt;", ">");
    
    let object: Vec<Value> = serde_json::from_str(to_html.as_str()).unwrap();
    let mut arr_data: Vec<HashMap<String,String>> = vec![];
    for element in &object{
        let mut data: HashMap<String, String> = HashMap::new();
        let parser = Html::parse_fragment(&element["title"].as_str().unwrap());
        let select_salles = &Selector::parse("span.salle").unwrap();
        let salle = parser.select(&select_salles).map(|x| x.inner_html()).collect::<String>();
        if salle.is_empty(){
            continue;
        }
        let date = &element["start"].to_string().replace("\\", "");
        data.insert("salle".to_string(), salle);
        data.insert("date".to_string(), date.to_string());
        arr_data.push(data);
    }
    let date_today: String = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
    let rdx: Vec<&HashMap<String,String>> = arr_data.iter() // je dedie cette variable à mon gars rdx merci à toi 
    .filter(|map| map.get("date").unwrap().contains(&date_today)).collect();
    let mut salles: String = String::new();
    if rdx.is_empty(){
        salles.push_str("Pas de cours !");
    }else {
        salles.push_str(rdx[0].get("salle").unwrap().to_owned().replace("[", "").replace("]", "").as_str());
    }
    Ok(salles)
}
#[tauri::command]
async fn set_presence(cookie: &str) -> Result<(),String> {
    let client = reqwest::Client::builder().build().map_err(|e| e.to_string())?;
    let resp = client.get("https://www.pepal.eu/presences").header("Cookie", cookie).send().await.map_err(|e| e.to_string())?; // récupère la page des notes  
    let body = resp.text().await.map_err(|e| e.to_string())?;
    let re = Regex::new("<td><a href=\"(.*?)\" class=\"btn btn-primary\"><i class=\"icon wb-list\"></i> <span class=\"hidden-sm-down\">Relevé de présence</span></a></td>").unwrap();

    let test = re.captures_iter(&body)
    .map(|story| {
        story[1].to_string()
    }).collect::<Vec<_>>();
    // println!("{:?}",test); // voir si tout se passe bien
    let presence_id = test.iter()
        .map(|element| {
            element.split("/")
            .skip(3).collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>();
    // println!("{:?}",presence_id); /: refer to l.64
    let mut param = HashMap::new();
    param.insert("act", "set_present"); // to set présent
    let seance_pk_idx = if is_past_noon() { // get seance id morning/after-noon
        println!("id de l'aprem");
        1
    } else {
        println!("id du matin");
        0
    };
    param.insert("seance_pk", presence_id[seance_pk_idx][0]);
    println!("{:?}",param);

    client.post("https://www.pepal.eu/student/upload.php").form(&param).send().await.map_err(|e| e.to_string())?; //valider la présence 
    //return true or false if presence is set
    Ok(())
}
fn is_past_noon() -> bool{
    let time_of_day = Local::now().time();
    let past_noon = NaiveTime::from_hms(12, 0, 0);
    if time_of_day > past_noon{
        true
    }else{
        false
    }
}
fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![get_notes,get_name,get_room,get_cookie,set_presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
