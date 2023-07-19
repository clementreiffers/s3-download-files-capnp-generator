ARG RELEASE_VERSION=35

FROM curlimages/curl AS downloader
ARG RELEASE_VERSION

RUN curl -O -L https://github.com/clementreiffers/s3-download-files-capnp-generator/releases/download/${RELEASE_VERSION}/s3-download-files-capnp-generator-release-alpine-v${RELEASE_VERSION}

FROM python:alpine AS runner
ARG S3_REGION

RUN pip install aws awscli_plugin_endpoint

COPY --from=downloader ./s3-download-files-capnp-generator ./

RUN chmod +x ./s3-download-files-capnp-generator
