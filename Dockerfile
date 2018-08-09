FROM ekidd/rust-musl-builder:stable as build
WORKDIR /workdir
USER root
COPY --chown=rust . .
RUN chown -R rust /workdir 
USER rust
RUN cargo build --release

FROM alpine
ENV SSL_CERT_DIR /etc/ssl/certs
RUN apk add --update ca-certificates
COPY --from=build /workdir/target/x86_64-unknown-linux-musl/release/salvage_job_kicker /usr/local/bin
CMD ["salvage_job_kicker"]
