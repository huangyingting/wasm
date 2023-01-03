# buildah bud --annotation "module.wasm.image/variant=compat-smart" -t huangyingting/rust-wasmedge .
FROM --platform=${BUILDPLATFORM} docker.io/library/rust:1.66.0 AS build
WORKDIR /opt/build
COPY . .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM scratch
COPY --from=build /opt/build/target/wasm32-wasi/release/rust-wasmedge.wasm .
CMD ["/rust-wasmedge.wasm"]