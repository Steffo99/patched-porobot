use std::env;
use matrix_sdk::{Client};
use matrix_sdk::config::{SyncRoomMessageEvent, SyncSettings};


#[tokio::main]
async fn main() -> ! {
	
	let homeserver_url = env::var("MATRIX_HOMESERVER_URL")
		.expect("MATRIX_HOMESERVER_URL to be defined");

	let user_id: String = env::var("MATRIX_USER_ID")
		.expect("MATRIX_USER_ID to be defined");
	let user_password: String = env::var("MATRIX_USER_PASSWORD")
    .expect("MATRIX_USER_PASSWORD to be defined");

    let client = Client::builder()
		.homeserver_url(homeserver_url)
		.build()
		.await?;
	
    client.login_username(&user_id, &user_password).send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    
    client.sync(SyncSettings::default()).await;
}