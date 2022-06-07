FROM rustlang/rust:nightly-slim

RUN apt update && apt install cmake openssl libssl-dev pkg-config -y

COPY dummy.rs .

# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock Cargo.toml .cargo ./

# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml

# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --release

# Now return the file back to normal
RUN sed -i 's/dummy.rs/src\/main.rs/' Cargo.toml

# Copy the rest of the files into the container
COPY src src

# Now this only builds our changes to things like src
RUN cargo build --release

# Run the binary
CMD ["./target/release/join-alerts"]