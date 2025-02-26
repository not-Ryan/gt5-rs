use gt5::autodiscover;

const USER_ID: u32 = 1;

#[tokio::main]
async fn main() {
    let gt5 = autodiscover().await.expect("Could nto get gt5");

    gt5.set_led(true).await.unwrap();

    println!("Deleting all users in db");
    if let Err(e) = gt5.delete_all().await {
        eprintln!("Could not delete all users: {e}");
    };

    gt5.enroll_start(USER_ID).await.expect("Enroll start");

    println!("-- Stage 1");
    println!("   Press finger ⬇");
    gt5.wait_pressed(true).await.unwrap();
    gt5.capture_finger(true).await.unwrap();
    gt5.enroll_x(1).await.unwrap();

    println!("   Release finger ⬆");
    gt5.wait_pressed(false).await.unwrap();

    println!("-- Stage 2");
    println!("   Press finger ⬇");
    gt5.wait_pressed(true).await.unwrap();
    gt5.capture_finger(true).await.unwrap();
    gt5.enroll_x(2).await.unwrap();

    println!("   Release finger ⬆");
    gt5.wait_pressed(false).await.unwrap();

    println!("-- Stage 3");
    println!("   Press finger ⬇");
    gt5.wait_pressed(true).await.unwrap();
    gt5.capture_finger(true).await.unwrap();
    gt5.enroll_x(3).await.unwrap();

    println!("   Release finger ⬆");
    gt5.wait_pressed(false).await.unwrap();

    println!("User created!");
    let template = gt5.get_template(USER_ID).await.unwrap();
    println!("Your template: {template}");

    gt5.set_led(false).await.unwrap();
}
