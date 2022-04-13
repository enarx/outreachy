
## Building python code with WASM
Today we'll learn how to build python code with WebAssembly using wasmtime.

![alt text](https://www.wasm.builders/remoteimages/uploads/articles/rwb0ucoz0olkjc0k2qq1.png)
### DOCKER INSTALLATION :
1. Uninstall old versions to avoid errors:

```

$ sudo apt-get remove docker docker-engine docker.io containerd runc
```
2. Install using the repository:

```
$ sudo apt-get update

$ sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
```
3. Add Docker's official GPG key and setup stable repository:

```
$ curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
$ echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
$(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```
4. Install Docker Engine:

```
$ sudo apt-get update
$ sudo apt-get install docker-ce docker-ce-cli containerd.io
$ sudo apt-get install docker-ce=5:20.10.14~3-0~ubuntu-focal docker-ce-cli=5:20.10.14~3-0~ubuntu-focal containerd.io
```
5. Verify that Docker Engine is installed by the command:

```
$ sudo docker run hello-world
```

### CPython's WASM build:
1. Clone the repo:

```
git clone https://github.com/singlestore-labs/python-wasi/
```
2. Go to the directory 'python-wasi' :
 

```
cd python-wasi
```
3. Build the docker image using this command:
 

```
docker build -f docker/Dockerfile -t wasi-build:latest docker
```
4. Let's start the docker container and mount the current and working directory inside the docker container by the command:

```
docker run -it --rm -v $(pwd):$(pwd) -w $(pwd) wasi-build:latest bash
```
5. To download the CPython source, dependencies and to build the CPython's WASM build, run:

```
./run.sh
```

### Wasmtime
Wasmtime is a small and efficient runtime for WebAssembly & WASI.

```
$ curl https://wasmtime.dev/install.sh -sSf | bash
$ wasienv install-sdk unstable
```

### Running the python code on wasmtime

Create a file named CounDigit.py and write the Program: 
```
def countDigit(n):
 count = 0
 while n != 0:
  n //= 10
  count += 1
 return count
n = 345289467
print("Number of digits : % d" % (countDigit(n)))
```
Run the program 
1. Switch to the root python-wasi directory.
2. Run the python code in wasmtime using the command:

```
wasmtime run - mapdir=$(pwd)/opt::opt \
 - opt/wasi-python/bin/python3.wasm -c "$(cat $(pwd)/CountDigit.py)"
```
Output:

```
Number of digits :9
```
Hope it helps! Do let me know in the comments!
