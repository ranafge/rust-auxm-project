mod deref_trait;
fn main() {
    println!("Hello, world!");
    deref_trait::deref_trait_understanding();


}
#[cfg(test)]
mod tests {
    #[test]
    fn work_nocapture()  {
        assert_eq!(2,2);
        println!("Yes work_nacapture function is workding.")
    
    }
}

