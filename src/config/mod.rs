//! Module defining configurable values that can be used by the binaries.
#![allow(missing_docs)]


/// Configuration required by the `exec` feature.
#[cfg(feature = "exec")]
pub mod exec {
	lazy_static::lazy_static! {
		/// The locale that card data should be downloaded in.
		/// 
		/// # Examples
		/// 
		/// - `en_US`
		/// - `it_IT`
		/// 
		pub static ref DATA_DRAGON_LOCALE: String = 
			micronfig::required("DATA_DRAGON_LOCALE");

		/// The set codes for which card data should be downloaded, separated by commas.
		/// 
		/// # Examples
		/// 
		/// - `set1,set2,set3`
		/// - `set1`
		/// - `set1,set2,set3,set4,set5,set6,set6cde,set7`
		/// 
		pub static ref DATA_DRAGON_SET_CODES: Vec<String> = 
			micronfig::required::<String>("DATA_DRAGON_SET_CODES")
				.split(",").map(|s: &str| s.to_string()).collect();
	}
}

/// Configuration required by the `jpg` feature.
#[cfg(feature = "jpg")]
pub mod jpg {
	lazy_static::lazy_static! {
		/// Secret key configured in imgproxy.
		pub static ref POROXY_KEY: Vec<u8> = micronfig::required("POROXY_KEY");

		/// Salt configured in imgproxy.
		pub static ref POROXY_SALT: String = micronfig::required("POROXY_SALT");

		/// URL where imgproxy can be reached at.
		pub static ref POROXY_HOST: String = micronfig::required("POROXY_HOST");
	}
}

/// Configuration required by the `discord` feature.
#[cfg(feature = "discord")]
pub mod discord {
	lazy_static::lazy_static! {
		/// ID of the guild where commands should be registered in.
		/// 
		/// If not defined, commands are registered globally, and then cached.
		/// 
		/// Useful for development, since guild commands are not cached.
		pub static ref SERENITY_DEV_GUILD_ID: Option<serenity::model::id::GuildId> = micronfig::optional("SERENITY_DEV_GUILD_ID").map(|s: u64| s.into());

		/// The Discord bot token.
		pub static ref SERENITY_TOKEN: String = micronfig::required("SERENITY_TOKEN");
		
		/// The Discord bot app ID.
		pub static ref SERENITY_APPID: u64 = micronfig::required("SERENITY_APPID");
	}
}