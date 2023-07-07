FROM ubuntu AS downloader

RUN apt-get update && apt-get install -y curl unzip

RUN curl -O -L https://github.com/clementreiffers/s3-download-files-capnp-generator/releases/download/24/release-alpine-v24.zip

RUN unzip release-alpine-v24.zip

FROM alpine AS runner

COPY --from=downloader ./s3-download-files-capnp-generator ./

RUN chmod +x ./s3-download-files-capnp-generator
