use lil_schemy::generate_openapi_debug;
use std::env;

fn main() {
    let paths = env::var_os("API_PATHS").unwrap();
    let paths = serde_json::from_str::<Vec<String>>(paths.to_str().unwrap()).unwrap();
    println!("Generating schemas for paths: {:?}", paths);
    match generate_openapi_debug(paths) {
        Ok(schema) => println!("{}", schema),
        Err(err) => println!("{}", err),
    }
}
