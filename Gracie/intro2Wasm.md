****WEB ASSEMBLY****

WebAssembly is an open standard that allows the execution of binary code on the web.
WebAssembly is a new type of code that can be run in modern web browsers — it is a low-level
assembly-like language with a compact binary format that runs with near-native performance and 
provides languages such as C/C++, 
C#,Java,Rust e.t.c, with a compilation target so that they can run on the web.
The main goal of WebAssembly is to enable high-performance applications on web pages.

**The benefits of using WebAssembly are:**

Near-native performance.

Security/Encryption.

Easy debugging.

Open-source.

Hardware, language, and platform-independent etc.

WebAssembly is designed from the ground up to solve the performance problem.
It can overcome bottleneck issues that traditional JavaScript wasn't meant to solve.
In WebAssembly, there's no need to parse and interpret code.

****USING WEB ASSEMBLY WITH JAVA****

There are numbers of programming language you can use with web assembly,
But I would be explaining web assembly using java. And I shall be making use of something called
JWebAssembly.

What is JWebAssembly: JWebAssembly is a Java bytecode to WebAssembly compiler. 
It uses Java class files as input. That it can compile any language that 
compile to Java bytecode like Clojure, Groovy, JRuby, Jython, Kotlin and Scala.
As output, it generates the binary format (.wasm file) or the text format (.wat file). The target is to run Java natively in the browser with WebAssembly.

Exporting functions
To export a Java function to make it accessible from JavaScript, you must add the annotation de.inetsoftware.jwebassembly.api.annotation.Export.

```import de.inetsoftware.jwebassembly.api.annotation.Export;
@Export
public static int add( int a, int b ) {
return a + b;
}
```
importing functions
To import a JavaScript function to make it accessible from Java, you must add the annotation de.inetsoftware.jwebassembly.api.annotation.Import. 
The method can be declared native or can have a Java implementation which will be ignored on compiling.

```
import de.inetsoftware.jwebassembly.api.annotation.Import;
@Import( module = "global.Math", name = "max" )
static int max( int a, int b) {
return Math.max( a, b );
}
```

JWebAssebmly, which  is a Java bytecode to WebAssembly compiler. It uses Java class files as input. That it can compile any language that compile to Java bytecode like Clojure, Groovy, JRuby, Jython, Kotlin and Scala.
As output ,it generates the binary format.

Note: Java values passed to WebAssembly 
exported functions are automatically downcasted
to WebAssembly values. Types are inferred at runtime, 
and casting is done automatically. Thus, a WebAssembly function
acts as any regular Java function.