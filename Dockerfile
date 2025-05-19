ARG BUILD_FROM
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)

# Install required packages
# Using Alpine package manager since all add-ons are based on Alpine Linux
RUN apk update && \
    apk add --no-cache \
      gst-rtsp-server \
      gstreamer \
      gst-plugins-bad \
      gst-plugins-base \
      gst-plugins-good \
      gst-plugins-bad \
      openssl

# Copy data for add-on
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]

# Test