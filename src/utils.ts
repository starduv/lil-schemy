import { sync as fg } from 'fast-glob';
import { readFileSync } from 'fs';
import ts, { CompilerOptions } from 'typescript';

let cache: ts.ModuleResolutionCache;

export const getAst = (cwd: string, options: CompilerOptions) => (module_ref: string, sourceFileName: string): string | undefined => {
    if (!cache) {
        cache = ts.createModuleResolutionCache(cwd, path => path, options);
    }

    let ast: ts.Node | undefined;
    // module_ref is an absolute path
    if (module_ref === sourceFileName) {
        ast = _getAst(module_ref, false);
    } else {
        // we're looking for a module reference found in another source file
        const resolution = ts.resolveModuleName(module_ref, sourceFileName, options, {
            fileExists: ts.sys.fileExists,
            readFile: ts.sys.readFile,
        }, cache);

        ast = _getAst(resolution.resolvedModule?.resolvedFileName as string, resolution.resolvedModule?.isExternalLibraryImport as boolean);
    }


    const astStr = JSON.stringify(ast || {});

    if ((ast as ts.SourceFile).fileName.endsWith('fastify.d.ts')) {
        console.log('No error, but no module either - ', astStr.length === 0)
    }

    return astStr;
};

export const getRootFiles = (cwd: string, globs: string[]): string[] => fg(globs, {
    absolute: true,
    cwd
});

const _getAst = (p: string, external: boolean): ts.Node => {
    const file = readFileSync(p, {
        encoding: 'utf-8',
        flag: 'r'
    });

    const ast = ts.createSourceFile(p, file, ts.ScriptTarget.ESNext, false, external ? ts.ScriptKind.External : ts.ScriptKind.TS);

    if (p.endsWith('fastify.d.ts')) {
        console.log(ast.fileName);
    }

    return ast;
};
