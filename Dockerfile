ARG BUILD_FROM
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)

# Install required packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      libgstrtspserver-1.0-0 \
      libgstreamer1.0-0 \
      libgstreamer-plugins-bad1.0-0 \
      gstreamer1.0-x \
      gstreamer1.0-plugins-base \
      gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad \
      libssl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy data for add-on
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]

# Test