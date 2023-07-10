FROM ubuntu AS downloader

RUN apt-get update && apt-get install -y curl unzip

RUN curl -O -L https://github.com/clementreiffers/s3-download-files-capnp-generator/releases/download/24/release-alpine-v24.zip

RUN unzip release-alpine-v24.zip

FROM python:alpine AS runner

RUN pip install aws awscli_plugin_endpoint

COPY --from=downloader ./s3-download-files-capnp-generator ./

RUN chmod +x ./s3-download-files-capnp-generator
