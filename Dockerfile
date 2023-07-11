ARG RELEASE_VERSION=34

FROM ubuntu AS downloader
ARG RELEASE_VERSION

RUN apt-get update && apt-get install -y curl unzip

RUN curl -O -L https://github.com/clementreiffers/s3-download-files-capnp-generator/releases/download/${RELEASE_VERSION}/release-alpine-v${RELEASE_VERSION}.zip

RUN unzip release-alpine-v${RELEASE_VERSION}.zip

FROM python:alpine AS runner
ARG S3_REGION

RUN pip install aws awscli_plugin_endpoint

COPY --from=downloader ./s3-download-files-capnp-generator ./

RUN chmod +x ./s3-download-files-capnp-generator
