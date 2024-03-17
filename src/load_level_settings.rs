pub mod LoadLevelSettings {
    use serde::Deserialize;
    use serde_json::{Map, Number};
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    pub struct LevelSettings {
        pub levels: HashMap<String, LevelData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct LevelData {
        pub types: HashMap<String, String>,
        pub speed: String,
    }

    pub fn read_settings() -> HashMap<String, LevelData> {
        let file = File::open("assets/level_settings.json").unwrap();
        let reader = BufReader::new(file);
        let level_settings: LevelSettings = serde_json::from_reader(reader).unwrap();
        level_settings.levels
    }
}
