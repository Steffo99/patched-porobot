FROM rust:1.64 AS files
WORKDIR /usr/src/patched_porobot
COPY . .

FROM files AS build
RUN cargo install --path . --all-features --bins

FROM debian:buster AS system
COPY --from=build /usr/local/cargo/bin/patched_porobot /usr/local/bin/patched_porobot

FROM system AS entrypoint
ENTRYPOINT ["bobbot"]
CMD []

FROM entrypoint AS final
LABEL org.opencontainers.image.title="Patched Porobot"
LABEL org.opencontainers.image.description="Legends of Runeterra card database utilities and bots"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"
LABEL org.opencontainers.image.url="https://github.com/Steffo99/patched-porobot"
LABEL org.opencontainers.image.authors="Stefano Pigozzi <me@steffo.eu>"
ENV RUST_LOG "warn,patched_porobot=info,patched_porobot_telegram=info,patched_porobot_discord=info,patched_porobot_matrix=info"
