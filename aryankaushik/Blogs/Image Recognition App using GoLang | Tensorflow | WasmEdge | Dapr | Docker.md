# Image Recognition App using GoLang | Tensorflow | WasmEdge | Dapr | Docker

![1]()

Hi Folks, welcome to another super interesting blog featuring another cool WebAssembly application.

It is an Image Recognition Application made using Go Language, works on a [Tensorflow]((https://www.tensorflow.org/)) model and it requires [Dapr](https://docs.dapr.io/) and [WasmEdge runtime](https://wasmedge.org/) for execution.

#### Small brief about these technologies:
- In order to build microservices, Dapr is a versatile framework. Containers are required to initialise Dapr, here we are using Docker.
- A WebAssembly VM, like WasmEdge, provides a secure and high-performance runtime for microservices.
- WebAssembly-based microservices can be written in a number of programming languages, including Rust, C/C++, Swift, and JavaScript.
- WebAssembly programs are embedded into Dapr sidecar applications and are therefore portable and agnostic to Dapr host environments.
- A WasmEdge SDK makes it easy to create Tensorflow inference microservices.
- WasmEdge is a Kubernetes compatible runtime and could play an important role as a lightweight container alternative to run microservices.

#### Let's get started:
1) We need to install Go, Docker, Dapr, WasmEdge.


- Go
```
sudo apt  install golang-go
```
- WasmEdge

```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

```
![2](https://www.wasm.builders/remoteimages/uploads/articles/2kaoejydqntawuhyx524.png)

- Docker

```
sudo apt-get remove docker docker-engine docker.io containerd runc
```
- Dapr

```
wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash

```

![3](https://www.wasm.builders/remoteimages/uploads/articles/xop3w8yxjeth4knqo8v8.png)


![4](https://www.wasm.builders/remoteimages/uploads/articles/27tmoyek4e7muvxghoei.png)

- Initialise Dapr

```
dapr init
```
![5](https://www.wasm.builders/remoteimages/uploads/articles/2jtvimiiq2cyrkngy26t.png)

- Check your Docker to verify

```
docker ps
```


![6](https://www.wasm.builders/remoteimages/uploads/articles/gtusec9dpzm00ajfo980.png)

2) Make a directory, for me its `imgpro_wasm`.

```
mkdir imgpro_wasm
cd imgpro_wasm/
```

![1](https://www.wasm.builders/remoteimages/uploads/articles/hetlhvz6dwxt3f4gpuej.png)


3) Clone this git repo created by [Second Stage](https://www.secondstate.io/).

```
git clone https://github.com/second-state/dapr-wasm.git
```

![7](https://www.wasm.builders/remoteimages/uploads/articles/e14pijxvsyykragkhn72.png)

4) Go to fuctions/classify and build the classify function 

```
cd functions/classify
 ./build.sh
```

![9](https://www.wasm.builders/remoteimages/uploads/articles/xqrj6oh69y5h3y2z342s.png)
 
5) Start Web Service for User GUI:

```
cd web-port
go build
./run_web.sh
```

![10](https://www.wasm.builders/remoteimages/uploads/articles/ixe8sgp9rk45c689catk.png)

6) Build and start the microservice for tensorflow-based image classification:

```
cd image-api-go
go build --tags "tensorflow image"
./run_api_go.sh
cd ../
```
![11](https://www.wasm.builders/remoteimages/uploads/articles/1m750w7gt9cyy3nqjeik.png)
 
7) The App is ready to Work
- Open your default browser and go to:

```
http://localhost:8080/static/home.html
```
![11](https://www.wasm.builders/remoteimages/uploads/articles/nkyc1jvfiwsu1qh8a7r6.png)
 
#### Hence the model will detech the objects in the images that will be uploaded to the application.

Wasm is an opportunity make more efficient, powerful, fast applications. Daily alot of Contributors are making a really good progress. Let's make much more interesting and cool applications using WebAssembly to make it more fun!

This ML model was created by Second State team. Do check their cool developments on their [github](https://github.com/second-state).

References:
1. https://wasmedge.org/book/en/start/install.html
2. https://docs.dapr.io/
3. https://github.com/second-state/dapr-wasm
4. https://www.infoq.com/articles/webassembly-dapr-wasmedge/
5. https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04
