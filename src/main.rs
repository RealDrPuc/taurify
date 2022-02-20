use dialoguer::Input;
use std::fs;
use std::path::Path;

// TO-DO: Make everything, just, nicer. Specially in the last part of the code, it sucks down there.

fn main() {
  let mut install_tauri_plugin: bool = true;
  if !Path::new("custom.js").exists() {
    println!("Couldn't find custom.js, not using the bad code part.");
    install_tauri_plugin = false;
  }

  let url: String = Input::new()
    .with_prompt("Enter the URL")
    .with_initial_text("https://")
    .validate_with(|input: &String| -> Result<(), &str> {
      if input.contains('.') && input.contains("http") {
          Ok(())
      } else {
          Err("The URL is incorrect.")
      }
    })
    .interact_text()
    .unwrap();

  // window.location.replace("
  let mut index_html: String = r#"<script> window.location.replace(""#.to_owned();
  // window.location.replace("urlhere")
  index_html = format!(r#"{}{}")"#, index_html, url);

  let project_path: String = Input::new()
    .with_prompt("Enter the path to your Tauri project (without ending in /)")
    .interact_text().unwrap();

  // TO-DO: This hopes src-tauri exists
  let tauri_conf_json = read_file(&format!("{}/src-tauri/tauri.conf.json", project_path));
  
  let tauri_conf = json::parse(&tauri_conf_json).unwrap();
  let relative_dist_dir = &tauri_conf["build"]["distDir"];

  // TO-DO: This hopes src-tauri exists. Remove unwrap()
  fs::write(format!("{}/src-tauri/{}/index.html", project_path, relative_dist_dir), index_html).unwrap();

  if !install_tauri_plugin {
    println!("Done! :)");
    std::process::exit(0);
  }

  let mut tauri_plugin = include_str!("tauri_plugin.rs").to_owned();
  let custom_js = read_file(&"./custom.js");
  
  // TO-DO: There has to be another way...
  tauri_plugin = format!(r##"{}{}"#);}}}}"##, tauri_plugin, custom_js);

  // TO-DO: This hopes src-tauri exists. Remove unwrap()
  fs::write(format!("{}/src-tauri/src/custom_js.rs", project_path), tauri_plugin).unwrap();
  
  let main_rs_path = format!("{}/src-tauri/src/main.rs", project_path);
  let main_rs = read_file(&main_rs_path);
  // I made this... I'm so sorry.
  let split_in_main = main_rs.split("fn main() {").collect::<Vec<_>>();
  let main_fn = split_in_main.get(1).unwrap();
  let split_in_builder = main_fn.split("context!())").collect::<Vec<_>>();
  let new_builder = format!("{}context!())\n.plugin(custom_js_plugin){}", split_in_builder.get(0).unwrap(), split_in_builder.get(1).unwrap());
  let new_main_rs = format!("{}fn main() {{\nlet custom_js_plugin = CustomJSPlugin::new();{}", split_in_main.get(0).unwrap(), new_builder);
  
  // Copy that mf, I don't trust that above shit
  fs::copy(format!("{}/src-tauri/src/main.rs", project_path), format!("{}/src-tauri/src/main.rs.bak", project_path)).unwrap();

  // TO-DO: This hopes src-tauri exists. Remove unwrap()
  fs::write(format!("{}/src-tauri/src/main.rs", project_path), new_main_rs).unwrap();

  println!("Done! :)");
}

fn read_file(file: &str) -> String {
  // TO-DO: Remove unwrap()
  String::from_utf8_lossy(
    &fs::read(file).unwrap()
  ).parse().unwrap()
}