####################################################################################################
## Builder
####################################################################################################

FROM rust:1.75 AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

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

COPY ./lstc-web .
COPY ./lstc-domain /lstc-domain

ENV RUSTFLAGS='-C link-arg=-s'
RUN cargo install --path .
RUN trunk build --release

####################################################################################################
## Final image
####################################################################################################

FROM nginx

WORKDIR /app
EXPOSE 80

COPY --from=builder /app/dist .
COPY --from=builder /app/nginx.conf /etc/nginx/nginx.conf
