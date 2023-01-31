# WASM Sample Applications

## Introduction
This repo contains several sample WASM applications based on different frameworks

- dotnet-spin, a .net core http echo WASM application based on fermyon spin[^1] framework
- go-spin, a go http echo WASM application based on fermyon spin[^1] framework
- rust-slight, a rust WASM application supports read/write/delete key value from redis cache, it is based on spiderlightning[^2] framework from deis lab
- rust-wasmedge, a rust http echo WASM application based on WasmEdge[^3]

[^1]: Spin is an open source framework for building and running fast, secure, and composable cloud microservices with WebAssembly.
[^2]: SpiderLightning defines a set of WebAssembly Interface Types (i.e., WIT) files that abstract distributed application capabilities, such as state management, pub/sub, event driven programming, and more.
[^3]: WasmEdge is a lightweight, high-performance, and extensible WebAssembly runtime for cloud native, edge, and decentralized applications

## Deployment
This repo also comes with kubernetes deploy manifests in `deploy` folder, to deploy those WASM applications into kubernetes, please follow [Run WASM applications from Kubernetes
](https://msazure.club/run-wasm-applications-from-kubernetes/)

## CI
Github CI is also included to build container images, refer to .github/workflows