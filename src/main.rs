use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
fn main() {
    //parameters which are tied to arguments
    let parameters = vec![
        "width=".to_string(),
        "height=".to_string(),
        "title=".to_string(),
        "option=".to_string(),
    ];
    let additional_settings=vec![
    "x-label".to_string(),
    ];
    let mut file = File::create("Settings.toml").unwrap();
    let mut args = env::args();
    let mut args_vec = Vec::<String>::new();
    for i in 0..=args.len() {
        match args.next() {
            Some(value) => args_vec.push(value),
            None => {
                args_vec.remove(0);
            }
        }
    }
    //saving 
    let mut config_vec = Vec::<String>::new();
    for i in 0..=parameters.len() - 1 {
        let content = &parameters[i];
        if content.contains("title") || content.contains("option")
        {
            let formatted = format!("\"{}\"", args_vec[i]);
            config_vec.push(content.to_string() + &formatted + "\n");
        } else {
            config_vec.push(content.to_string() + &args_vec[i] + "\n");
        }
    }
    let config_content = config_vec.concat();
    file.write_all(&config_content.into_bytes()).unwrap();
    //Continuing the process 
    Command::new("cargo")
    .args(["make","whole-process"])
    .spawn()
    .expect("Error occured");

}
