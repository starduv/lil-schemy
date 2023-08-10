use lil_schemy::generate_openapi_debug;

fn main() {
    let paths = vec![
        "/Users/joelrainear-wills/dev/lil-schemy/tests/test-api/routes/animals.ts".to_string(),
        "/Users/joelrainear-wills/dev/lil-schemy/tests/test-api/routes/user.ts".to_string(),
    ];

    match generate_openapi_debug(paths) {
        Ok(schema) => println!("{}", schema),
        Err(err) => println!("{}", err),
    }
}
