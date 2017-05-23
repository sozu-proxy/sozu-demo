FROM alpine:edge

COPY . /source/

ENV PORT=8080
ENV ASSETS_DIR=/var/www/assets/

RUN apk add --no-cache --virtual .build-dependencies \
  cargo \
  g++ \
  gcc \
  musl-dev \
  rust && \
  apk add --no-cache openssl-dev llvm-libunwind && \
  cd /source && \
  cargo build --release && \
  cp /source/target/release/rust-demo-app /bin/rust-demo-app && \
  mkdir -p /var/www/ && \
  cp -r /source/assets /var/www/ && \
  cd / && \
  apk del .build-dependencies && \
  apk del && \
  rm -rf /source && \
  rm -rf /root/.cargo

EXPOSE 8080

CMD ["/bin/rust-demo-app"]
