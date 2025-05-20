## How to run locally

### Building Docker Image Locally

#### Building with HA's Official Build Tool

This is downloads another docker container and builder the image inside of that container

`docker run --rm -it --name builder --privileged -v /Users/samkumar/Development/SK-Productions-LLC/Home-Assistant/evolink-addon/evolink:/data -v /var/run/docker.sock:/var/run/docker.sock:ro ghcr.io/home-assistant/armv7-builder -t /data --all --test -i evolink-addon-armv7 -d local`

#### Building with Host Machine's Docker

`docker build --build-arg BUILD_FROM="ghcr.io/home-assistant/armhf-base:latest" -t local/evolink-addon .`

### Running Docker Image Locally

docker run --rm -v /tmp/my_test_data:/data local/evolink-addon-armv7

docker run --rm -v /tmp/my_test_data:/data -p PORT_STUFF_IF_NEEDED local/evolink-addon-armv7
