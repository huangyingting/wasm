# docker buildx build --platform wasi/wasm32 -t huangyingting/go-spin go-spin

FROM --platform=${BUILDPLATFORM} tinygo/tinygo:0.25.0 AS build
WORKDIR /opt/build
COPY . .
RUN tinygo build -wasm-abi=generic -target=wasi -gc=leaking -no-debug -o main.wasm main.go

FROM scratch
COPY --from=build /opt/build/main.wasm .
COPY --from=build /opt/build/spin.toml .
