ARG BUILD_FROM=ghcr.io/home-assistant/aarch64-base:latest 
# TODO: Remove default value which is currenly suppressing warning
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)


# Alpine Image with Rust Port

# Set Environment Variables    
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=stable

# Install dependencies
RUN apk add --no-cache \
    ca-certificates \
    gcc \
    gstreamer \
    gstreamer-tools \
    gstreamer-dev \
    gst-plugins-base \
    gst-plugins-good \
    gst-plugins-bad \
    gst-libav

# TODO: Implement the architecture logic to download the correct Rust version

# Install Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_VERSION

# Set working directory and copy source code
WORKDIR /usr/local/src/neolink
COPY . /usr/local/src/neolink

# Build Neolink
RUN ls -als

COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]


