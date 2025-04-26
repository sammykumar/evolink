ARG BUILD_FROM
FROM $BUILD_FROM 

# All Addons are based on Alphine Linux Image Automatically (https://developers.home-assistant.io/docs/add-ons/configuration#add-on-dockerfile)

# Copy data for add-on
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]

# Docker Build
# docker run \
# 	--rm \
# 	--privileged \
# 	-v ~/.docker:/root/.docker \
# 	-v /my_addon:/data \
#     ghcr.io/home-assistant/amd64-builder:latest \
# 		--all \
# 		-t /data