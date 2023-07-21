use firebase_auth_sdk::FireAuth;

#[tokio::main]
async fn main() {
    //let auth = FireAuth::new("YoURpr0j3ct1D");
    let auth = FireAuth::emulator("localhost:9099");

    let email = "some@email.com";
    let pass = "myPassword";

    let user = auth.sign_in_email(email, pass, true).await.unwrap();
    println!("{user:?}");

    let oob_code_result = auth.verify_email(&user.id_token).await.unwrap();
    println!("oob code: {oob_code_result:?}");

    let oob_code_result = auth.reset_password(email).await.unwrap();
    println!("oob code: {oob_code_result:?}");
}
