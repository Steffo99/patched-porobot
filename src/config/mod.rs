//! Module defining configurable values that can be used by the binaries.
#![allow(missing_docs)]


/// Configuration required by the `exec` feature.
#[cfg(feature = "exec")]
pub mod exec {

	/// The locale that card data should be downloaded in.
	///
	/// # Examples
	///
	/// - `en_US`
	/// - `it_IT`
	///
	micronfig::required!(DATA_DRAGON_LOCALE, String);

	/// The set codes for which card data should be downloaded, separated by commas.
	///
	/// # Examples
	///
	/// - `set1,set2,set3`
	/// - `set1`
	/// - `set1,set2,set3,set4,set5,set6,set6cde,set7`
	///
	micronfig::required!(DATA_DRAGON_SET_CODES, String);
}

/// Configuration required by the `jpg` feature.
#[cfg(feature = "jpg")]
pub mod jpg {

	/// Secret key configured in imgproxy.
	micronfig::required!(POROXY_KEY, Vec<u8>);

	/// Salt configured in imgproxy.
	micronfig::required!(POROXY_SALT, String);

	/// URL where imgproxy can be reached at.
	micronfig::required!(POROXY_HOST, String);
}

/// Configuration required by the `discord` feature.
#[cfg(feature = "discord")]
pub mod discord {

	/// ID of the guild where commands should be registered in.
	///
	/// If not defined, commands are registered globally, and then cached.
	///
	/// Useful for development, since guild commands are not cached.
	micronfig::optional!(SERENITY_DEV_GUILD_ID, serenity::model::id::GuildId);

	/// The Discord bot token.
	micronfig::required!(SERENITY_TOKEN, String);

	/// The Discord bot app ID.
	micronfig::required!(SERENITY_APPID, u64);
}