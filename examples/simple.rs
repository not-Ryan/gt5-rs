use gt5::autodiscover;

#[tokio::main]
async fn main() {
    let gt5 = autodiscover().await.expect("Could nto get gt5");

    match gt5.get_enroll_count().await {
        Ok(count) => println!("Enrolled count: {count}"),
        Err(err) => eprintln!("Could not get enrolled count: {err}"),
    }
}
