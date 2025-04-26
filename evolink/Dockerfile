ARG BUILD_FROM
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)

# Install requirements for Alpine Image with Rust
RUN apk add --no-cache \
    ca-certificates \
    gcc

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=%%RUST-VERSION%%

# Copy data for add-on
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]


# docker run --rm -it --name builder --privileged -v /Users/samkumar/Development/SK-Productions-LLC/Home-Assistant/evolink-addon/evolink:/data -v /var/run/docker.sock:/var/run/docker.sock:ro ghcr.io/home-assistant/armv7-builder -t /data --all --test -i evolink-addon-armv7 -d local
