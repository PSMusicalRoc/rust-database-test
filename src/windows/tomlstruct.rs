use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::env::current_exe;


#[derive(Deserialize, Serialize)]
pub struct LoginData {
    pub ip: String,
    pub user: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct WindowSettings {
    pub height: u32,
    pub width: u32
}

#[derive(Deserialize, Serialize)]
pub struct TomlData {
    pub firstlogin: bool,
    pub login: LoginData,
    pub windowsettings: WindowSettings
}

impl Default for TomlData {
    fn default() -> Self {
        TomlData {
            firstlogin: true,
            login: LoginData {
                ip: "test-ip.com".to_string(),
                user: "test-user".to_string(),
                password: "test-password".to_string()
            },
            windowsettings: WindowSettings {
                height: 1080,
                width: 1920
            }
        }
    }
}

pub fn ensure_data_exists(path: &str) -> Result<(), String> {
    let mut filepath = current_exe().unwrap();
    filepath.pop();
    filepath.push(path);

    if fs::read(filepath).is_err() {
        let data: TomlData = TomlData {..Default::default()};

        return write_tomldata(path, &data);        
    }

    Ok(())
}

pub fn load_tomldata(path: &str) -> TomlData {

    let mut filepath = current_exe().unwrap();
    filepath.pop();
    filepath.push(path);

    match fs::read_to_string(filepath) {
        Ok(contents) => {
            match toml::from_str(&contents.as_str()) {
                Ok(data) => data,
                Err(err) => {
                    panic!("{}", err.to_string());
                }
            }
        },
        Err(err) => {
            panic!("{}", err.to_string());
        }
    }
}

pub fn write_tomldata(path: &str, data: &TomlData) -> Result<(), String> {
    
    let mut filepath = current_exe().unwrap();
    filepath.pop();
    filepath.push(path);

    match toml::to_string_pretty(data) {
        Ok(msg) => {
            match fs::write(filepath, msg) {
                Ok(_) => { return Ok(()); },
                Err(_) => { return Err("Could not write to file".to_string());}
            }
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
}