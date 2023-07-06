IMG = clementreiffers/s3-downloader-capnp-generator

docker-build:
	docker build -t $(IMG) .

docker-push:
	docker push $(IMG)
