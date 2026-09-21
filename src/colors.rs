pub mod colors {
    use rand::random_range;
    use std::collections::HashMap;

    pub type Num = i32;
    pub type _STR = String;

    pub fn r_value(index: Num) -> Num {
        random_range(1..index + 1)
    }

    pub fn get_hash() -> HashMap<Num, _STR> {
        let mut hashes: HashMap<Num, _STR> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "R".to_string());
        hashes
    }

    pub fn get_r() -> HashMap<Num, _STR> {
        get_hash()
    }

    pub fn get_hash_g() -> HashMap<Num, _STR> {
        let mut hashes: HashMap<Num, _STR> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "G".to_string());
        hashes
    }

    pub fn get_g() -> HashMap<Num, _STR> {
        get_hash_g()
    }

    pub fn get_hash_b() -> HashMap<Num, _STR> {
        let mut hashes: HashMap<Num, _STR> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "B".to_string());
        hashes
    }

    pub fn get_b() -> HashMap<Num, _STR> {
        get_hash_b()
    }
}