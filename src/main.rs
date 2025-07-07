use pcb_parsing::parse_to_s_expr::parse_dsn_to_s_expr;



fn main() {
    let data = std::fs::read_to_string("specctra_test.dsn").unwrap();
    let result = match parse_dsn_to_s_expr(&data) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            panic!("Failed to parse the DSN file");
        }
    };
}
