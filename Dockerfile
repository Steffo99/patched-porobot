FROM rust:1.64-buster AS files
WORKDIR /usr/src/patched_porobot
COPY . .

FROM files AS system
RUN apt install -y libssl1.1

FROM system AS build
RUN cargo install --path . --all-features --bins

FROM build AS entrypoint
ENTRYPOINT []
CMD []

FROM entrypoint AS final
LABEL org.opencontainers.image.title="Patched Porobot"
LABEL org.opencontainers.image.description="Legends of Runeterra card database utilities and bots"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"
LABEL org.opencontainers.image.url="https://github.com/Steffo99/patched-porobot"
LABEL org.opencontainers.image.authors="Stefano Pigozzi <me@steffo.eu>"
ENV RUST_LOG "warn,patched_porobot=info,patched_porobot_telegram=info,patched_porobot_discord=info,patched_porobot_matrix=info"
