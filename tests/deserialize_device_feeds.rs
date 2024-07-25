use std::{env, fs::{self, File}, io::Read};

use wzdx::model::df::DeviceFeed;

#[test]
fn scenarios() {
    let current_dir = env::current_dir().unwrap();
    let scenarios_path = format!("{}/tests/resources/scenarios/device_feeds", current_dir.to_str().unwrap());
    for entry in fs::read_dir(scenarios_path).unwrap() {
        let mut f = File::open(entry.unwrap().path().to_str().unwrap()).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        let _: DeviceFeed =  serde_json::from_str(&contents).unwrap();
    }
}
