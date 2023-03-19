FROM --platform=linux/amd64 rust:1.68-bullseye AS builder
ARG TARGETPLATFORM
ARG RUSTTARGET

RUN apt-get update && \
    apt-get upgrade --assume-yes

RUN \
    echo "Building for ${TARGETPLATFORM}." && \
    if [ ${TARGETPLATFORM} = "linux/arm64" ]; then \
        apt-get install --assume-yes gcc-aarch64-linux-gnu; \
    fi && \
    if [ ${TARGETPLATFORM} = "linux/arm/v7" ]; then \
        apt-get install --assume-yes gcc-arm-linux-gnueabihf; \
    fi

RUN rustup target add ${RUSTTARGET}

WORKDIR /usr/src/patched_porobot/
COPY ./ ./

RUN find /usr/bin/arm-linux-gnueabihf-gcc

RUN cargo build --all-features --bins --release --target=${RUSTTARGET}

#############################################################################

FROM --platform=${TARGETPLATFORM} rust:1.68-slim-bullseye AS runtime
ARG RUSTTARGET

WORKDIR /usr/src/patched_porobot/
COPY --from=builder \
    /usr/src/patched_porobot/target/${RUSTTARGET}/release/patched_porobot_discord \
    /usr/src/patched_porobot/target/${RUSTTARGET}/release/patched_porobot_telegram \
    /usr/src/patched_porobot/target/${RUSTTARGET}/release/patched_porobot_matrix \
    /usr/bin/

ENTRYPOINT []
CMD []

LABEL org.opencontainers.image.title="Patched Porobot"
LABEL org.opencontainers.image.description="Legends of Runeterra card database utilities and bots"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"
LABEL org.opencontainers.image.url="https://github.com/Steffo99/patched-porobot"
LABEL org.opencontainers.image.authors="Stefano Pigozzi <me@steffo.eu>"
ENV RUST_LOG "warn,patched_porobot=info,patched_porobot_telegram=info,patched_porobot_discord=info,patched_porobot_matrix=info"
