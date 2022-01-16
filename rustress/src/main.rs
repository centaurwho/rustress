use rustress::load;


fn main() {
    let map = load(String::from("./rustress"));

    for (key, val) in map {
        println!("{:?}: {:?}", key, val);
    }
}