extern crate hashmap;
use hashmap::HashMap;

fn main() {
    let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();
    // use the values stored in map

    for (k, v) in timber_resources {
        match k {
            "Norway" => assert_eq!(v, 100),
            "Denmark" => assert_eq!(v, 50),
            "Iceland" => assert_eq!(v, 10),
            _ => unreachable!(),
        }
    }
}
