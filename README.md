<p align="center" style="font-weight:900;font-size:25px;">Lil' Schemy</p>
<p align="center">
<img src="./lil-schemy.png" width="175px" style="border-radius:50px;margin:auto;"/>
</p>
<p align="center">"Open-Api, Json-Schemer, input, Typescript, ouput, Beamer"</p>

---

[![License](http://img.shields.io/:license-mit-blue.svg?style=flat)](https://opensource.org/licenses/MIT)

### What is it?
Lil' Schemy is a cli tool that enables "code first" schema generation. Use it to generate an OpenApi v3.0.3 schema from your TypeScript project. Focus on building a well tested, functionally correct product, then tack on a schema.

### Supported Platforms
Lil Schemy is a [Node addon]. The supported platforms/architectures are:
- Linux - x86_64 | aarch64
- Windows - x86_64 | aarch64
- Apple - x86_64 | aarch64

### Upcoming Features
__Generate schemas from return type of functions__
```JS
getDriverRoutes = async (): Promise<[string, boolean, Array<number>]> => {
    return ["sup", true, [1,2,3]];
};

const response = LilResponse(await getDriverRoutes())
```

## [LICENSE](LICENSE)

[Node addon]:(https://github.com/neon-bindings/neon)