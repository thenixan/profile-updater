FROM ekidd/rust-musl-builder as builder

#RUN apk upgrade --update-cache --available && \
#    apk add openssl && \
#    rm -rf /var/cache/apk/*


ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine

RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/profile-updater \
    /usr/local/bin/

ENV YT_API_KEY ''

ENV YT_CHANNEL_ID ''

CMD /usr/local/bin/profile-updater