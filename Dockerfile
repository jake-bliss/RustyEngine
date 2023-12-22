# Use an official Rust runtime as a parent image
FROM rustlang/rust:nightly as builder

# Set the working directory in the container to /app
WORKDIR /app

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Set DATABASE_URL environment variable
ENV DATABASE_URL=mysql://staging_db_user_developer:Staging358!@/cloudsql/krato-admin:us-central1:krato-staging/rustenginemysql
ENV TOKEN=phr93AjbU7bXkwd0tuRKCnn58A 

# This build step will cache your dependencies
RUN cargo build --release
RUN rm -rf src/

# Copy your source tree
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/commission_engine*
RUN cargo build --release

# Our second stage, that will be the final image
FROM debian:bullseye-slim 

# Set DATABASE_URL environment variable
ENV DATABASE_URL=mysql://staging_db_user_developer:Staging358!@/cloudsql/krato-admin:us-central1:krato-staging/rustenginemysql
ENV TOKEN=phr93AjbU7bXkwd0tuRKCnn58A 

# We need to add the target architecture of Rust binaries
# If you are using stable, you might change this to stable
# If you are using nightly, you might change this to nightly
ARG ARCH=x86_64-unknown-linux-gnu

# We install the SSL certificates so we can make HTTPS requests
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/commission_engine /usr/local/bin

# Set the startup command to run your binary
CMD ["commission_engine"]