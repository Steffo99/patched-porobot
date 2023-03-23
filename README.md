# ![](icon.png) Patched Porobot

Legends of Runeterra game data crate and chat bots

## Links

[![Telegram Bot](https://img.shields.io/badge/telegram%20bot-done-success)](https://t.me/patchedporobot)
 
[![Discord Bot](https://img.shields.io/badge/discord%20bot-done-success)](https://discord.com/api/oauth2/authorize?client_id=1071989978743193672&scope=applications.commands)
 
![Matrix Bot](https://img.shields.io/badge/matrix%20bot-to%20do-inactive)

[![Crates.io](https://img.shields.io/crates/v/patched_porobot)](https://crates.io/crates/patched_porobot)
 
[![Documentation](https://img.shields.io/docsrs/patched_porobot)](https://docs.rs/patched_porobot/0.9.2/patched_porobot/)
 
[![Chat](https://img.shields.io/matrix/patched_porobot:ryg.one?server_fqdn=matrix.ryg.one)](https://matrix.to/#/#patched_porobot:ryg.one)

## Screenshots

### Telegram bot

<details>
<summary>The bot's profile, seen when a chat with @patchedporobot is opened. The legal boilerplate has been shortened to fit the 120 characters limit of the Description field.</summary>

![](media/td-profile.png)

</details>

<details>
<summary>The message the bot sends when it detects an interaction from the user, such as the default /start command.</summary>

![](media/td-start.png)

</details>

<details>
<summary>The card search prompt that appears when attempting to use the bot in a chat.</summary>

![](media/td-prompt.png)

</details>

<details>
<summary>A search for "poro". Many poros are displayed, and also Braum Level 2, since it contains "poro" in its description.</summary>

![](media/td-poro.png)

</details>

<details>
<summary>The message sent when a card is clicked from the menu. It contains both the card image and a plain text render of the card (for accessibility). Additionally, the flavor text, the artist name, and a link to the full illustration are provided.</summary>

![](media/td-message.png)

</details>

## Licenses

Patched Porobot isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

<details>
<summary>List of licenses as output by cargo license</summary>

- **(Apache-2.0 OR MIT) AND BSD-3-Clause** (1): encoding_rs
- **(MIT OR Apache-2.0) AND Unicode-DFS-2016** (1): unicode-ident
- **0BSD OR Apache-2.0 OR MIT** (1): adler
- **AGPL-3.0-or-later** (1): patched_porobot
- **Apache-2.0** (2): fail, varint-rs
- **Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT** (1): wasi
- **Apache-2.0 OR BSL-1.0** (1): ryu
- **Apache-2.0 OR ISC OR MIT** (4): hyper-rustls, rustls, rustls-pemfile, sct
- **Apache-2.0 OR MIT** (154): ahash, anyhow, arc-swap, async-trait, autocfg, base64, base64, bitflags, block-buffer, bumpalo, cc, cfg-if, chrono, cpufeatures, crc32fast, crossbeam-channel, crossbeam-deque, crossbeam-epoch, crossbeam-utils, crypto-common, digest, downcast-rs, either, env_logger, erasable, fastrand, flate2, fnv, form_urlencoded, fs2, futures, futures-channel, futures-core, futures-executor, futures-io, futures-macro, futures-sink, futures-task, futures-util, generator, getrandom, glob, hashbrown, hermit-abi, hermit-abi, http, httparse, httpdate, humantime, ident_case, idna, indexmap, ipnet, itertools, itertools, itoa, js-sys, lazy_static, libc, lock_api, log, md5, memmap2, mime, num-integer, num-traits, num_cpus, once_cell, oneshot, parking_lot, parking_lot_core, percent-encoding, pin-project, pin-project-internal, pin-project-lite, pin-utils, ppv-lite86, pretty_env_logger, proc-macro-error, proc-macro-error-attr, proc-macro2, quick-error, quote, rand, rand_chacha, rand_core, rayon, rayon-core, rc-box, regex, regex-syntax, remove_dir_all, reqwest, rustc-hash, rustc_version, rustversion, scoped-tls, scopeguard, semver, serde, serde_derive, serde_json, serde_urlencoded, serde_with_macros, sha-1, signal-hook-registry, smallvec, socket2, stable_deref_trait, syn, tempfile, thiserror, thiserror-impl, thread_local, time, time-core, time-macros, tokio-rustls, tungstenite, typenum, unicase, unicode-bidi, unicode-normalization, url, utf-8, uuid, version_check, wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-futures, wasm-bindgen-macro, wasm-bindgen-macro-support, wasm-bindgen-shared, wasm-streams, web-sys, winapi, winapi-i686-pc-windows-gnu, winapi-x86_64-pc-windows-gnu, windows, windows-sys, windows-sys, windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc, windows_aarch64_msvc, windows_i686_gnu, windows_i686_gnu, windows_i686_msvc, windows_i686_msvc, windows_x86_64_gnu, windows_x86_64_gnu, windows_x86_64_gnullvm, windows_x86_64_msvc, windows_x86_64_msvc
- **Apache-2.0 OR MIT OR MPL-2.0** (1): htmlescape
- **Apache-2.0 OR MIT OR Zlib** (3): miniz_oxide, tinyvec, tinyvec_macros
- **BSD-3-Clause** (2): instant, never
- **BSD-3-Clause OR MIT** (1): rust-stemmers
- **Custom License File** (2): ring, webpki
- **ISC** (3): serenity, typemap_rev, untrusted
- **MIT** (62): aquamarine, async-tungstenite, atty, bitpacking, bytes, census, combine, convert_case, crunchy, darling, darling_core, darling_macro, dashmap, data-encoding, derive_more, dptree, fastfield_codecs, generic-array, h2, http-body, hyper, levenshtein_automata, loom, lru, lz4_flex, matchers, memoffset, mime_guess, mio, murmurhash32, nu-ansi-term, ordered-float, overload, ownedbytes, redox_syscall, serde-value, sharded-slab, slab, spin, strsim, take_mut, takecell, tantivy, tantivy-bitpacker, tantivy-common, tantivy-query-grammar, teloxide, teloxide-core, tokio, tokio-macros, tokio-stream, tokio-util, tower-service, tracing, tracing-attributes, tracing-core, tracing-log, tracing-subscriber, try-lock, valuable, want, winreg
- **MIT OR Unlicense** (8): aho-corasick, byteorder, memchr, regex-automata, tantivy-fst, termcolor, utf8-ranges, winapi-util
- **MPL-2.0** (1): webpki-roots
- **Unlicense** (1): measure_time
- **zlib-acknowledgement** (1): fastdivide

</details>