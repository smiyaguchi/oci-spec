#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod runtime;
mod serdes;

#[cfg(test)]
mod tests {
    use crate::runtime::{Spec, State};
    use std::collections::HashMap;
    use std::io::{Write, BufWriter};

    #[test]
    fn test_load() {
        match Spec::load("config.json") {
            Ok(_) => {},
            Err(e) => panic!("{}", e),    
        }
    }

    #[test]
    fn test_save() {
        let spec = match Spec::load("config.json") {
            Ok(s) => s,
            Err(e) => panic!("{}", e),    
        };

        match Spec::save(&spec, "config.json") {
            Ok(_) => {},
            Err(e) => panic!("{}", e),    
        }
    }

    #[test]
    fn test_writer() {
        let buf = Vec::new();
        let mut writer = BufWriter::new(buf);
        let state = State {
            version: "1.0.1-dev".to_string(),
            id: 1000.to_string(),
            status: "created".to_string(),
            pid: Some(1),
            bundle: "/test".to_string(),
            annotations: Some(HashMap::new()),    
        };

        match State::to_writer(&state, writer.by_ref()) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),    
        }
    }

    #[test]
    fn test_string() {
        let state = State {
            version: "1.0.1-dev".to_string(),
            id: 1000.to_string(),
            status: "created".to_string(),
            pid: Some(1),
            bundle: "/test".to_string(),
            annotations: Some(HashMap::new()),
        };
        
        match State::to_string(&state) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),    
        }    
    }
}
