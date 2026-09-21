pub mod colors {
    use std::collections::HashMap;
    use rand::random_range;

    pub fn r_value(index: i32) -> i32 {
        random_range(1..index + 1)
    }

    fn get_hash() -> HashMap<i32, String> {
        let mut hashes: HashMap<i32, String> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "R".to_string());
        hashes
    }

    pub fn get_r() -> HashMap<i32, String> {
        get_hash()
    }

    pub fn get_hash_g() -> HashMap<i32, String> {
        let mut hashes: HashMap<i32, String> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "G".to_string());
        hashes
    }

    pub fn get_g() -> HashMap<i32, String> {
        get_hash_g()
    }

    pub fn get_hash_b() -> HashMap<i32, String> {
        let mut hashes: HashMap<i32, String> = HashMap::new();
        let color_code = r_value(254);
        hashes.insert(color_code, "B".to_string());
        hashes
    }

    pub fn get_b() -> HashMap<i32, String> {
        get_hash_b()
    }
}