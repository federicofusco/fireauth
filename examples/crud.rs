use firebase_auth_sdk::FireAuth;

#[tokio::main]
async fn main() {
    //let auth = FireAuth::new("YoURpr0j3ct1D");
    let auth = FireAuth::emulator("localhost:4101");

    let email = "user@email.com";
    let pass = "my_initial_password";

    let created = auth.sign_up_email(email, pass, true).await.unwrap();
    println!("created: {}", &created.email);

    let refresed = auth.refresh_id_token(&created.refresh_token).await.unwrap();
    println!("refreshed: {refresed:?}");

    let created_checked = auth.get_user_info(&created.id_token).await.unwrap();
    println!("created checked: {created_checked:?}");

    let claims = auth.verify_id_token(&created.id_token).await.unwrap();
    println!("{claims:?}");

    let signed_in = auth.sign_in_email(email, pass, true).await.unwrap();
    println!("signed_in: {:?}", signed_in);

    let claims = auth.verify_id_token(&signed_in.id_token).await.unwrap();
    println!("{claims:?}");

    let new_password = "my_new_password";

    let updated_user = auth
        .change_password(&signed_in.id_token, new_password, true)
        .await
        .unwrap();
    println!("updated_user: {:?}", updated_user);

    auth.sign_in_email(email, new_password, true).await.unwrap();
}
