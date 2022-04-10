

## TUTORIAL ON HOW TO COMPILE C/C++ TO WEB ASSEMBLY AND RUN IN A BROWSER

### INTRODUCTION
WebAssembly is a new type of code that can be run in modern browsers and provides new feature and major gains in performance. 
It is not intended to be writteng by hand, rather it is designed to be an effective compilation target for source languages like c/c++, c#, rust etc.
In this tutorial i will be demostrating how to compile a c application to web assembly using Emscripten tool and how to run on a browser using vscode live server


### SETTING UP EMSCRIPTING WORK ENVIROMENT
Open your terminal, then copy and paste the following commands on your terminal. this is done once, skip if you have already done this setup before
##### Get the emsdk repo cloned to a directory 
    git clone https://github.com/emscripten-core/emsdk.git


##### Enter that directory
    cd emsdk


##### Fetch the latest version of the emsdk (not needed the first time you clone)
    git pull


##### Download and install the latest SDK tools.
    ./emsdk install latest


##### Make the "latest" SDK "active" for the current user. (writes .emscripten file)
    ./emsdk activate latest


##### Activate PATH and other environment variables in the current terminal
    source ./emsdk_env.sh
    
###### Reference site: https://emscripten.org/docs/getting_started/downloads.html


####  
#### COMPILING C CODE TO WASM
    

```C
#include <stdio.h>
#include <string.h>

int main() {
  int x = 0;

  while (x <= 10)
  {
    printf("hello, world! : %d\n\r", x);    
    x++;
  }
  
  return 0;
}

```
open the terminal fron emsdk/ folder and run the commands below. 
##### Make the "latest" SDK "active" for the current user. (writes .emscripten file)
    ./emsdk activate latest


##### Activate PATH and other environment variables in the current terminal
    source ./emsdk_env.sh

Here i Compiled my C application to wasm and creating HTML to run our code in, plus all the JavaScript "glue" code needed to run the wasm in the web environment.
Now.

Using the terminal window I used to enter the Emscripten compiler environment, I navigated to the directory i saved my c source file(), and ran the following command: 

    emcc hello.c -s WASM=1 -o hello.html

**-s WASM=1** — Specifies that we want wasm output. If we don’t specify this, Emscripten will just output asm.js, as it does by default.

**-o hello.html** — Specifies that we want Emscripten to generate an HTML page to run our code in (and a filename to use), as well as the wasm module and the JavaScript "glue" code to compile and instantiate the wasm so it can be used in the web environment.
Emscripten will generate the following file in the c source directory as show in the next slide.


####     
#### C SOURCE DIRECTORY WILL NOW CONTAIN A .html, .js, AND .wasm FILE AS SHOWN IN THE PICTURE BELOW
![image](https://user-images.githubusercontent.com/42975388/138512345-8d045da5-dded-4824-95f0-201182e356d8.png)

####   
#### SETTING UP LOCAL SERVER ON MY PC
I’ll need a server to run the hello.html generated  file on my browser.
My code editor of choice is vscode. So, I installed live server extension in my vscode editor program. 
![image](https://user-images.githubusercontent.com/42975388/138513394-c8ee63b9-9a70-4eb7-ad6f-6dd84b5a6d6f.png)
Reference site: https://github.com/ritwickdey/vscode-live-server


#### 
#### RUNNING ON A WEB BROWSER. 
To run the wasm file on a browser, i opened the hello.html on vscode, right  clicked on it and clicked on “open with live server” as shown below
![image](https://user-images.githubusercontent.com/42975388/138513509-b384cf13-fa01-4641-bf22-7be88cbf7986.png)


#### 
#### THE PICTURE BELOW SHOWS MY C APPLICATION RUNNING ON A WEB BROWSER

![image](https://user-images.githubusercontent.com/42975388/138513593-f08b8877-facd-4515-bbf5-3a75a1da6b42.png)

