<p align="center" style="font-weight:900;font-size:25px;">Lil' Schemy</p>
<p align="center">
<img src="./lil-schemy.png" width="200px" style="border-radius:50px;margin:auto;"/>
</p>
<p align="center">"Open-Api, Json-Schemer, input, Typescript, ouput, Beamer"</p>

---

[![License](http://img.shields.io/:license-mit-blue.svg?style=flat)](https://opensource.org/licenses/MIT)

<!-- ### Table of Contents
- [What is it](#whatisit)
- [How It Works](#howitworks)
- [Supported Platforms](#supportedplatforms)
- [Roadmap](#Roadmap) -->

### What is it?
Lil' Schemy is a cli tool that enables "code first" schema generation. Use it to generate an OpenApi v3.0 schema from your TypeScript project. Focus on building a well tested, functionally correct product, then tack on a schema.

### How It Works
Lil' Schemy works by finding Lil' functions and types, generating schemas from relevant symbols found or referenced within them. First, you need to indicate which files contain your route handlers by updating `schemy-config.js` like this...
```js
module.exports = {
    openApi: {
        // ... other stuff

        entry: [
            // this is an array of globs
            "./src/io/api-routes/**/*.ts"
        ],
        
        // ... other stuff
    }
}
```
Lil' Schemy will search the files that match the glob patterns you defined. Each invocation of `LilPath` found will result in the creation of a distinct [Path](https://swagger.io/specification/#paths-object) in your OpenApi schema. The first argument of `LilPath` is your route handler. It returns your route handler, so you can use it with frameworks like Express. Here's an example:
```TS
app.get('/', LilPath((request, reply): void => {
    reply.send();
}, {
    method: 'GET',
    path: '/',
}));
```
The second argument defines path properties, like `method` and `path`. LilSchemy isn't able to detect such things from your framework of choice. Instead is asks youf for a lil' help. This is an open source project, maybe someone will add method and path detection for your framework, who knows. There are other types to learn about int the API section, that allow to specify parameters, responses, and data types for your schema paths.

### Supported Platforms
This is a [Node addon]. The supported platforms/architectures are:
- Linux - x86_64 | aarch64
- Windows - x86_64 | aarch64
- Apple - x86_64 | aarch64

### Roadmap
- Add JSON Schema support

## [LICENSE](LICENSE)

### Notes
---

[Node addon]:(https://github.com/neon-bindings/neon)
