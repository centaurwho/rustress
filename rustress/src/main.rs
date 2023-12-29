use rustress::load;

fn main() {
    let map = load(String::from("./rustress")).unwrap();

    for (key, val) in map {
        println!("{:?}: {:?}", key, val);
    }
}
