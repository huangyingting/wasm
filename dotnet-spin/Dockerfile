# docker buildx build --platform wasi/wasm32 -t huangyingting/dotnet-spin dotnet-spin

FROM --platform=${BUILDPLATFORM} mcr.microsoft.com/dotnet/sdk:7.0 AS build
RUN apt-get update && apt-get install xz-utils 
RUN wget -qO- https://github.com/bytecodealliance/wizer/releases/download/v1.6.0/wizer-v1.6.0-x86_64-linux.tar.xz | tar -xvJ --strip-components=1 -C /usr/local/bin
RUN wget -qO- https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz | tar -xvz --strip-components=1 -C /usr/local
WORKDIR /opt/build
COPY . .
RUN dotnet build -c Release
RUN wasm-opt -O bin/Release/net7.0/DotnetSpin.wasm -o bin/Release/net7.0/DotnetSpin.wasm

FROM scratch
COPY --from=build /opt/build/bin/Release/net7.0/DotnetSpin.wasm /bin/Release/net7.0/DotnetSpin.wasm
COPY --from=build /opt/build/spin.toml .