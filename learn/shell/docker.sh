#!/usr/bin/bash

docker container commit container repository[:tag]
docker image push name[:tag]
docker image pull name[:tag]
docker image tag source-image[:tag] target-image[:tag]
docker image rm image
# FROM scratch
# COPY fs /
