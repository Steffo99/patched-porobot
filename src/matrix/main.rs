use std::env;
use matrix_sdk::{Client, LoginBuilder};
use matrix_sdk::config::{SyncSettings};
use matrix_sdk::ruma::events::room::message::{SyncRoomMessageEvent};


#[tokio::main]
async fn main() -> ! {
	
	let homeserver_url = env::var("MATRIX_HOMESERVER_URL")
		.expect("MATRIX_HOMESERVER_URL to be defined");

	let user_id: String = env::var("MATRIX_USER_ID")
		.expect("MATRIX_USER_ID to be defined");

	let shared_secret: String = env::var("MATRIX_SHARED_SECRET")
    	.expect("MATRIX_SHARED_SECRET to be defined");

    let client = Client::builder()
		.homeserver_url(homeserver_url)
		.build()
		.await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    
	loop {
		client.sync(SyncSettings::default()).await;
	}
}