####################################################################################################
## Builder
####################################################################################################

FROM rust:1.75 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=user
ENV UID=10001

WORKDIR /app

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . ./lstc-api

WORKDIR /app/lstc-api

ENV RUSTFLAGS='-C link-arg=-s'
RUN cargo install --path .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################

# Bookworm is Debian 12, which inludes glibc compatibility with rust:1.75 (above)
FROM debian:bookworm

WORKDIR /app
EXPOSE 8080

RUN apt-get update && \
    apt-get -y install curl && \
    rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy our build
COPY --from=builder /app/lstc-api/target/release/lstc-api .

# Use an unprivileged user.
USER user:user

CMD ["./lstc-api"]
