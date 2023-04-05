# Rust
FROM rust:latest as build

# Install dependencies
RUN apt-get -qq update

RUN apt-get install -y -q \
    clang \
    llvm-dev \
    libclang-dev \
    cmake \
    openssl

RUN cargo install diesel_cli --no-default-features --features postgres

# Set default user
RUN USER=root cargo new --bin people_data_api
WORKDIR /people_data_api

# Copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy over migrations and templates
COPY ./migrations ./migrations
COPY ./templates ./templates

# Copy dummy data (.csv) files
COPY ./seeds ./seeds

# This build to cache dependencies
RUN cargo build --release
RUN rm src/*.rs 

# Copy source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/people_data_api*
RUN cargo build --release

# Final base
FROM rust:latest

# Copy final build artifact
COPY --from=build /people_data_api/target/release/people_data_api .

# Copy dummy data (.csv) files
COPY --from=build /people_data_api/seeds seeds
# Copy templates
COPY --from=build /people_data_api/templates templates


EXPOSE 8080

# Set startup command

CMD ["./people_data_api"]