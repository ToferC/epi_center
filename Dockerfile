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

# Copy over migrations
COPY ./migrations ./migrations
COPY ./templates ./templates

# Copy dummy data
COPY ./names.csv ./names.csv
COPY ./org_structure.csv ./org_structure.csv

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
COPY --from=build /usr/local/cargo/bin/diesel .

COPY --from=build /people_data_api/names.csv .
COPY --from=build /people_data_api/org_structure.csv .
COPY --from=build /people_data_api/migrations migrations


EXPOSE 8080

# Set startup command

# CMD ./diesel migration run && ./people_data_api
CMD ./people_data_api