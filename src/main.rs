use llm_docs_rs;
fn main() {
    let result = llm_docs_rs::main();

    match result {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
