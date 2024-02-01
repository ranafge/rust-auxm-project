pub fn deref_trait_understanding() {
    let card: Option<&str> = Some("Rust book");
    if let Some(book) = card  {
        println!("I can read the book: {}", book)
    }
    let card2:Option<String> = Some("RUST BOOK ".to_string());
    // as_deref trait convert option<String> to option<&str>
    if let Some(book) = card2.as_deref()  {
        println!("I CAN READ THIS BOOK {}", book);
    }
    println!("CARD2 {}", card2.unwrap());
}