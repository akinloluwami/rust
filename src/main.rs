use std::path;

// mod print;
// mod vars;
// mod types;
mod strings;

fn main() {
    // print::print();
    // vars::vars();
    // types::types();
    // strings::strings();

    let url = "https://www.npmjs.com/package/electron-context-menu";
    let paths: Vec<&str> = url.split("/").collect();
    println!("{:?}", paths);
}
