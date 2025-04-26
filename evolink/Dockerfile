ARG BUILD_FROM=ghcr.io/home-assistant/aarch64-base:latest 
# TODO: Remove default value which is currenly suppressing warning
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)


# Alpine Image with Rust Port

# Install dependencies
RUN apk add --no-cache \
    ca-certificates \
    gcc

# Set Environment Variables    
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=%%RUST-VERSION%%


# TODO: Implement the architecture logic to download the correct Rust version

# Install Rust
# RUN set -eux; \
#     rustArch="armv7-unknown-linux-musleabihf"; \
#     rustupVersion="1.26.0"; \
#     rustupSha256="b8b8e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3e2c3"; \
#     url="https://static.rust-lang.org/rustup/archive/${rustupVersion}/${rustArch}/rustup-init"; \
#     wget "$url"; \
#     echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
#     chmod +x rustup-init; \
#     ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
#     rm rustup-init; \
#     chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
#     rustup --version; \
#     cargo --version; \
#     rustc --version;

# Copy data for add-on
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]


