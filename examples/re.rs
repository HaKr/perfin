use regex::Regex;

fn main() {
    let txt = "Datum/Tijd: ~datum~ ~tijd~";
    let re = Regex::new("~([^~]+)~").unwrap();

    for x in re.captures_iter(txt) {
        println!("X={:?}", x.get(1).unwrap().as_str());
    }
}
