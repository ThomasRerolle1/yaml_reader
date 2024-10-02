use ma_proc::{double, yaml_reader};

fn main() {
    let number = 5;
    let x = double!(number);
    println!("the double is {}", x);
    let file = yaml_reader!("data.yaml");
    println!("{:?}", file);
    let file = yaml_reader!("data1.yaml");
    println!("{:?}", file);
    let no_file = yaml_reader!("bad_file.yaml");
    println!("{:?}", no_file);
}
