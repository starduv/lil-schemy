<p align="center" style="font-weight:900;font-size:25px;">Lil' Schemy</p>
<p align="center">
<img src="./lil-schemy.png" width="200px" style="border-radius:50px;margin:auto;"/>
</p>
<p align="center">"Open-Api, Json-Schemer, input, Typescript, ouput, Beamer"</p>

---

[![License](http://img.shields.io/:license-mit-blue.svg?style=flat)](https://opensource.org/licenses/MIT)

- [What is it?](#what-is-it)
- [How It Works](#how-it-works)
- [Examples](#examples)
- [CLI](#cli)
- [API](#api)
  - [LilPath\<Func\>(fn: Func, options: PathItemOptions) : Func](#lilpathfuncfn-func-options-pathitemoptions--func)
  - [PathItemOptions](#pathitemoptions)
  - [OperationMethod](#operationmethod)
  - [LilResponse\<T\>(response: T, options: ResponseOptions) : T](#lilresponsetresponse-t-options-responseoptions--t)
  - [ResponseOptions](#responseoptions)
  - [LilBodyParam\<Param, Required\>](#lilbodyparamparam-required)
  - [LilHeader\<Param, Required, Format\>](#lilheaderparam-required-format)
  - [LilQueryParam\<Param, Required, Format\>](#lilqueryparamparam-required-format)
  - [LilRouteParam\<Param, Required, Format\>](#lilrouteparamparam-required-format)
  - [LilRequiredProp](#lilrequiredprop)
  - [LilSub\<From, To\>](#lilsubfrom-to)
  - [format](#format)
  - [NumberFormat](#numberformat)
  - [StringFormat](#stringformat)
  - [generate(cwd: string, config: LilSchemyOptions) : SchemyResult](#generatecwd-string-config-lilschemyoptions--schemyresult)
  - [LilSchemyOptions](#lilschemyoptions)
  - [OpenApiOptions](#openapioptions)
  - [LilSchemyResult](#lilschemyresult)
  - [OpenApiResult](#openapiresult)
- [Programmatic Use](#programmatic-use)
- [Supported Platforms](#supported-platforms)
- [MIT](#mit)


## What is it?
Lil' Schemy is a cli tool that enables "code first" schema generation. Use it to generate an OpenApi v3.1.x schema from your TypeScript project. Focus on building a well tested, functionally correct product, then tack on a schema.

## How It Works
Lil' Schemy works by finding Lil' functions and types, generating schemas from relevant symbols found or referenced within them. By the way, you can use the CLI to generate a default `schemy-config.js`. First, you need to indicate which files contain your route handlers by updating `schemy-config.js` like this...
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
Lil' Schemy will search the files that match the glob patterns you defined. Each invocation of `LilPath` found will result in the creation of a distinct OpenApi [path](https://swagger.io/specification/#paths-object). The first argument of `LilPath` is your route handler. It returns your route handler, so you can use it with frameworks like Express. Here's an example:
```TS
app.get('/', LilPath((request, reply): void => {
    reply.send();
}, {
    method: 'GET',
    path: '/',
}));
```
The second argument defines path properties, like `method` and route `path`. Lil' Schemy isn't able to detect such things from your framework of choice. Instead it asks you for a lil' help. This is an open source project, maybe someone will add method and route path detection for your framework, who knows. There are other functions and types to learn about in the API section that allow you to specify parameters, responses, and data types for your schema paths.

## Examples
There are several examples found in the [mock api](tests/test-api/routes/user.ts) used for testing.

## CLI
Here is the top level documentation. Run the CLI in a terminal to learn more.
```
Usage: lil-schemy [options] [command]

CLI to generate schemas from TypeScript

Options:
  -V, --version       output the version number
  --cwd <cwd>         base dir, for relative paths (default: "/Users/joelrainear-wills/dev/lil-schemy")
  -h, --help          display help for command

Commands:
  init                Create default lil-schemy configuration (schemy-config.js)
  generate [options]  Generate one or more schemas
  help [command]      display help for command
```

## API
### LilPath\<Func>(fn: Func, options: PathItemOptions) : Func
`LilPath` identifies an OpenApi path that needs documentation. All of the other Lil functions and types are used inside of this. It has two parameters:
- **fn**: A function that serves as a route handler in your application.
- **options**: An instance of the interface, `PathItemOptions`.

### PathItemOptions
`PathItemOptions` is an interface that represents options for an OpenApi path. It has the following properties:
- **method**: An instance of the enumeration type, `OperationMethod`.
- **path**: A string representing the route path.
- **tags** (optional): An array of strings. Tags are a way to categorize your paths. UI tools often group your paths together by tag.

### OperationMethod
`OperationMethod` is an enumeration type that represents an HTTP method. It can be one of the following: `'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE'.

### LilResponse\<T>(response: T, options: ResponseOptions) : T
`LilResponse` tells Schemy where to begin searching for the response type returned by your route handler. It has two parameters:
- **response**: A response value, object, or null.
- **options**: An instance of the interface, `ResponseOptions`.

### ResponseOptions
`ResponseOptions` is an interface that represents options for a response. It has the following properties:
- **description**: A string representing the description.
- **example** (optional): A string that Schemy converts into a reference like, `#/components/examples/<your string here>`. Schemy assumes you placed the corresponding example in `schemy-config.js`.
- **statusCode**: A number representing the status code.
  
### LilBodyParam<Param, Required>
`LilBodyParam` is a type that represents a body parameter. It has two parameters:
- **Param**: The type of the parameter.
- **Required** (optional): A boolean value indicating whether the parameter is required.

### LilHeader<Param, Required, Format>
`LilHeader` is a type that represents a header parameter. It has three parameters:
- **Param**: The type of the parameter.
- **Required** (optional): A boolean value indicating whether the parameter is required.
- **Format** (optional): An instance of the type, `format`.

### LilQueryParam<Param, Required, Format>
`LilQueryParam` is a type that represents a query parameter. It has three parameters:
- **Param**: The type of the parameter.
- **Required** (optional): A boolean value indicating whether the parameter is required.
- **Format** (optional): An instance of the type, `format`.

### LilRouteParam<Param, Required, Format>
`LilRouteParam` is a type that represents a route parameter. It has three parameters:
- **Param**: The type of the parameter.
- **Required** (optional): A boolean value indicating whether the parameter is required.
- **Format** (optional): An instance of the type, `format`.

### LilRequiredProp<T>
`LilRequiredProp` is a type that represents a required property.
- **T**: The type of the parameter whose name is listed as a required property.

### LilSub<From, To>
`LilSub` replaces your design time type with a desired schema type:
- **From**: The type used during design time of your application.
- **To**: The type used to generate an OpenApi schema.  
"Why?" Let's say you have a request handler that returns a type that directly or indirectly references a type from a module that Schemy can't resolve. In that case, you can tell Schemy to use a different type that Schemy can resolve. In the example below, Schemy will not discover that the TS module `node:events` lives in the file `events.d.ts`.
```TS
// events.d.ts
declare module 'node:events' {
    import events = require('events');
    export = events;
}

// stream.d.ts
declare module stream {
    import { EventEmitter, Abortable } from 'node:events';
    // ...
}

// return type of route handler
export interface Resolvable {
    stream: LilSub<stream, Uint8Array>
}
```

### format
`format` is a type that represents a format. It can be either a `StringFormat` or a `NumberFormat`.

### NumberFormat
`NumberFormat` is a type that represents a number format. It can be one of the following: `int32`, `int64`, `float`, or `double`.

### StringFormat
`StringFormat` is a type that represents a string format. It can be one of the following: `date-time`, `time`, `date`, `duration`, `email`, `idn-email`, `hostname`, `idn-hostname`, `ipv4`, `ipv6`, `uuid`, `uri`, `uri-reference`, `uri-template`, `json-pointer`, `relative-json-pointer`, `regex`, `iri`, or `iri-reference`.

### generate(cwd: string, config: LilSchemyOptions) : SchemyResult
`generate` is a function that generates schemas, optionally writing schemas to user defined filepaths.
- **cwd**: The base directory used for relative paths when finding files and resolving modules.
- **options**: An instance of the interface, `LilSchemyOptions`.

### LilSchemyOptions
`LilSchemyOptions` is a type that represents desired schemas
- **openApi** (optional): An instance of the type, `OpenApiOptions`

### OpenApiOptions
`OpenApiOptions` tells Lil' Schemy to generate an OpenApi schema
- **base**: A user defined OpenApi schema that will overlay the generated schema. The only required field is `openapi`, which is always version "3.1.0" (for now)
- **output** (optional): The filepath where Lil' Schemy should write the schema. It will not write the schema without this.
- **entry**: an array of blob patterns describing the files containing http paths that need schemas.

### LilSchemyResult
`LilSchemyResult` is a type containing the result of schema generation
- **openApi** An instance of the type `OpenApiResult`

### OpenApiResult
`OpenApiResult` contains the resultant schema and a filepath where the schema was written
- **schema**: The schema as a string
- **filepath**: The filepath where the schema was written

## Programmatic Use
You can use Lil-Schemy from your own module by calling the `generate` function. Here's an example:
```TS
const config = {
    openApi: {
        // All values in base take precedence over generated values.
        base: {
            info: {
                title: "My Application",
                version: "1.0.0"
            },
            components: {
                securitySchemes: {
                    oauth2: {
                        type: "oauth2",
                        description: "Get a JWT",
                        flows: {
                            authorizationCode: {
                                authorizationUrl: "https://www.myauthservice.come/auth",
                                tokenUrl: "https://www.myauthservice.come/auth/token",
                                scopes: {
                                    email: "I know your email address"
                                }
                            }
                        }
                    }
                },
            },
            security: [
                {
                    oauth2: []
                }
            ]
        },
        // Glob patterns to modules declaring api paths.
        entry: [
            "./src/routes/**/*.ts"
        ],
        // Where the resultanat OpenApi schema is written.
        output: "../dist/public/openapi.json"
    }
};
const result = generate(__dirname, config);

console.log("The OpenApi schema was written here: ", result.openApi?.filepath);
console.log("I'll write it to standard output for your convenience...");
console.log(result.openApi?.schema);
```

## Supported Platforms
This is a [Node addon]. The supported platforms/architectures are:
- Linux - x86_64 | aarch64
- Windows - x86_64 | aarch64
- Apple - x86_64 | aarch64

## [MIT](LICENSE)

[Node addon]:(https://github.com/neon-bindings/neon)
