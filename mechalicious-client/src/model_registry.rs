use std::collections::HashMap;
use std::{fs::File, io::Read};
use vectoracious::Model;

use super::*;

pub struct ModelRegistry {
    base_path: PathBuf,
    models: HashMap<&'static str, Model>,
}

impl ModelRegistry {
    pub fn new(base_path: PathBuf) -> ModelRegistry {
        ModelRegistry {
            base_path,
            models: HashMap::new(),
        }
    }

    pub fn get_model(&mut self, model_path: &'static str) -> &Model {
        self.models.entry(model_path).or_insert_with(|| {
            let mut infile =
                File::open(self.base_path.join(model_path)).expect("could not find file");
            let mut whole_file = String::new();
            infile
                .read_to_string(&mut whole_file)
                .expect("could not read file");
            dbg!(&whole_file);
            let model = Model::from_v2d(&whole_file).expect("Unable to parse v2d file");
            model
        })
    }
}
