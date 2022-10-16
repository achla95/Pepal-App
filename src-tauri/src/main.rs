#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::collections::HashMap;
use scraper::{Html,Selector};
use reqwest;
use serde_json::Value;
use tauri_plugin_store::PluginBuilder;
use chrono;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
async fn get_notes(username: &str, password: &str) -> Result<Vec<Vec<String>>, String> { 
    let mut login_info: HashMap<&str,&str> = HashMap::new();
    login_info.insert("login", username);
    login_info.insert("pass", password);
    let client = reqwest::Client::builder().cookie_store(true).build().map_err(|e| e.to_string())?;
    client.post("https://www.pepal.eu/include/php/ident.php").form(&login_info).send().await.map_err(|e| e.to_string())?;
    let req = client.get("https://www.pepal.eu/?my=notes").send().await.map_err(|e| e.to_string())?;
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
async fn get_room(username: &str, password : &str) -> Result<String,String> {
    let mut login_info: HashMap<&str,&str> = HashMap::new();
    login_info.insert("login", username);
    login_info.insert("pass", password);
    let client = reqwest::Client::builder().cookie_store(true).build().map_err(|e| e.to_string())?;
    client.post("https://www.pepal.eu/include/php/ident.php").form(&login_info).send().await.map_err(|e| e.to_string())?;
    let req = client.get("https://www.pepal.eu/?my=edt").send().await.map_err(|e| e.to_string())?;
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
fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![get_notes,get_name,get_room])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
