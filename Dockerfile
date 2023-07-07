FROM ubuntu AS downloader

RUN apt-get update && apt-get install -y curl unzip

RUN curl -O -L https://github.com/clementreiffers/s3-download-files-capnp-generator/releases/download/21/release-linux-v21.zip

RUN unzip release-linux-v21.zip

FROM ubuntu AS runner

COPY --from=downloader ./s3-download-files-capnp-generator /usr/local/bin
