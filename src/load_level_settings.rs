pub mod load_level_settings {
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;

    #[derive(Debug, Deserialize)]
    pub struct LevelSettings {
        pub levels: HashMap<String, LevelData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct LevelData {
        pub types: HashMap<String, String>,
        pub speed: String,
        pub fires: String,
    }

    pub fn read_settings() -> HashMap<String, LevelData> {
        let file = File::open("assets/level_settings.json").unwrap();
        let reader = BufReader::new(file);
        let level_settings: LevelSettings = serde_json::from_reader(reader).unwrap();
        level_settings.levels
    }
}
