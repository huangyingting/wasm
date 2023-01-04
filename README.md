# Run WASM applications from Kubernetes

## Inspect .wasm import section
```shell
wasm-objdump --section 'import' --details  path/to/file.wasm
```

## WasmEdge Kubernetes
The crun project has WasmEdge support baked in. 

### Install WasmEdge
```shell
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -e all -v 0.11.2 -p /usr/local
```
### Build crun with WasmEdge support
Install building tools and libraries
```shell
sudo apt update
sudo apt install -y make git gcc build-essential pkgconf libtool \
    libsystemd-dev libprotobuf-c-dev libcap-dev libseccomp-dev libyajl-dev \
    go-md2man libtool autoconf python3 automake
```
Next, configure, build, and install a crun binary with WasmEdge support.

```shell
git clone https://github.com/containers/crun
cd crun
./autogen.sh
./configure --with-wasmedge
make
sudo make install
```

Add crun as a containerd shim to start and run WasmEdge application

```shell
vi /etc/containerd/config.toml
```

```toml
        [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun]
          runtime_type = "io.containerd.runc.v2"
          privileged_without_host_devices = false
          pod_annotations = ["*.wasm.*", "wasm.*", "module.wasm.image/*", "*.module.wasm.image", "module.wasm.image/variant.*"]
          [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.crun.options]
            BinaryName = "crun"
```

```shell
systemctl restart containerd
systemctl status containerd
```

Add a `RuntimeClass` to support WasmEdge crun containerd shim
```yaml
# wasmedge-crun-v2.yaml
apiVersion: node.k8s.io/v1
handler: crun
kind: RuntimeClass
metadata:
  name: wasmedge-crun-v2
scheduling:
  nodeSelector:
    kubernetes.azure.com/wasmedge-crun-v2: "true"
```

```shell
kubectl apply -f wasmedge-crun-v2.yaml
kubectl label nodes <NODE> kubernetes.azure.com/wasmedge-crun-v2=true
```

## Containerd Wasm Shims

### Pre-requisite
Install `containerd-wasm-shims` to your Kubernetes work nodes by following [Using a shim in Kubernetes](https://github.com/deislabs/containerd-wasm-shims#using-a-shim-in-kubernetes)

### Build and deploy
Github action will automatically build and push the image to ghcr.io.

go-spin.yaml has everything to deploy golang wasm application to Kubernetes cluster
Run `kubectl apply -f deploy/go-spin.yaml`

dotnet-spin.yaml has everything to deploy csharp wasm application to Kubernetes cluster
Run `kubectl apply -f deploy/dotnet-spin.yaml`

To run applications locally, make sure [spin](https://github.com/fermyon/spin) is installed
Then change directory to `go-spin` or `dotnet-spin`, run `spin build` and `spin up`.
