#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod runtime;
mod serdes;

#[cfg(test)]
mod tests {
    use crate::runtime::Spec;

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
}
