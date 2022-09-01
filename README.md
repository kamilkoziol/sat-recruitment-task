# SAT recruitement task

Repository contains APIs prepared for SAT recruitement task.

Stack:

`rust`

## Running app

### 0. Running precompiled app

Repository contains precompiled app that can be executed right away **HOWEVER** application was compiled on Ubuntu 20.04 and will run only on Linux distributions (no Windows and not sure about MacOS tho).

If you want to use precompiled app move forward to step **4**

### 1. Install rust

To check if Rust is already installed in your OS open a terminal and execute below command:

`rustc --version`

If Rust is missing follow official guides suitable for your OS. To validate installation reopen terminal and execute above command once again. Expected result :

`rustc 1.63.0 (4b91a6ea7 2022-08-08)` (version might vary)

### 2. Clone repository

Create new project directory, move to that directory and execute command:

`https://github.com/kamilkoziol/sat-recruitment-task.git` to clone repository.

### 3. Building app

Open terminal, move to project directory and execute command

`cargo build --release`

### 4. Running app

To run an app go to project directory and execute:

`./sat-rust-api` for precompiled app,

`./target/release/sat-rust-api` for app compiled locally,

### 5. Setting custom port number

App runs on port `8080` by default. You can specify other port by either passing port as and argument while running an app:

`./target/release/sat-rust-api 8081`

or by setting environment variable

- `export SAT_API_PORT=8082` for Linux/macOS
- `set SAT_API_PORT=8082` for Windows

### 6. Validate installation

To validate installation execute
`curl --location --request GET 'localhost:8080/probabilityOfUnitInjectorFail?VIN=foovin' --header 'Cache-Control: no-cache' --header 'Accept: */*' --header 'Accept-Encoding: gzip, deflate' --header 'Connection: keep-alive'`

Execution should result in getting response:
`{"failProbability":"0.08"}` (number might vary)
