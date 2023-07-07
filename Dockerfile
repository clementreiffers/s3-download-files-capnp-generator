FROM ubuntu AS downloader

RUN apt-get update && apt-get install -y curl unzip

RUN curl -O -L linkhere

RUN unzip filehere

FROM ubuntu AS runner

COPY --from=downloader ./fake-cf-api ./
