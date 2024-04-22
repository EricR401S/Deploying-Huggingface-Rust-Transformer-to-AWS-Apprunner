FROM rust:latest as builder
ENV APP infermp10
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

# Use the testing debian
FROM debian:testing
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP

# Copy the database file into the container
# ADD https://vgexo.s3.us-west-1.amazonaws.com/vgtest-polo.db /
ADD https://vgexo.s3.us-west-1.amazonaws.com/open_llama_3b-q4_0-ggjt.bin /

# Export this Actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["infermp10"]