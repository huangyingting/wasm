# Go WASM application

## Pre-requisite
Install `containerd-wasm-shims` to your Kubernetes work nodes by following [Using a shim in Kubernetes](https://github.com/deislabs/containerd-wasm-shims#using-a-shim-in-kubernetes)

## Build and deploy
Github action will automatically build and push the image to ghcr.io.

go-spin.yaml has everything to deploy golang wasm application to Kubernetes cluster
Run `kubectl apply -f deploy/go-spin.yaml`

dotnet-spin.yaml has everything to deploy csharp wasm application to Kubernetes cluster
Run `kubectl apply -f deploy/dotnet-spin.yaml`

To run applications locally, make sure [spin](https://github.com/fermyon/spin) is installed
Then change directory to `go-spin` or `dotnet-spin`, run `spin build` and `spin up`.