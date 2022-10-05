# https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/
	
FROM rust:latest 
 
RUN apt update; apt upgrade -y 
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
 
# RUN rustup target add aarch64-unknown-linux-gnu
# RUN rustup target add x86_64-apple-darwin
# RUN rustup target add aarch64-apple-darwin
# RUN rustup target add x86_64-unknown-linux-gnu

# RUN rustup toolchain install stable-aarch64-unknown-linux-gnu
# RUN rustup toolchain install stable-aarch64-apple-darwin

WORKDIR /app 

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

#CMD ["cargo", "build", "--target", "aarch64-unknown-linux-gnu"]
#CMD ["cargo","build","--release"]
#RUN cargo build --target=aarch64-apple-darwin
RUN cargo build --release

RUN ls target/release

#CMD ["cargo","build","--release","--all-targets"]
