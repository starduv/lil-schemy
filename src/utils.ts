import { sync as fg } from 'fast-glob';
import { appendFileSync, readFileSync, writeFileSync } from 'fs';
import ts, { ImportDeclaration, SourceFile } from 'typescript';

export interface TypeShiftContext {
    rootFiles: string[],
    getAst: (path: string) => ts.Node;
}

type RefPaths = [string | undefined, string | undefined];

let cache: ts.ModuleResolutionCache;

export const getContext = (cwd: string, globs: string[], includeParent: boolean = false, compilerOptions: ts.CompilerOptions): TypeShiftContext => {
    if (!cache) {
        cache = ts.createModuleResolutionCache(
            cwd,
            path => path,
            compilerOptions
        );
    }

    const astMap = new Map();
    const masterCache: Map<string, ts.Node> = new Map();
    const files = fg(globs, {
        absolute: true,
        cwd
    });

    const fileNames: Array<RefPaths> = files.map(fullPath => [undefined, fullPath]);

    while (fileNames.length) {
        const [rel, full] = fileNames.pop() as [string | null, string];
        if (!masterCache.has(full)) {
            masterCache.set(full, getAst(full, includeParent));
        }

        const ast = masterCache.get(full);
        astMap.set(rel || full, ast);

        for (const paths of getRefModules(ast as ts.Node, compilerOptions)) {
            if (paths[0] && paths[1]) {
                fileNames.push(paths as RefPaths);
            }
        }
    }

    console.debug("generated ast(s) for ", astMap.keys());

    return { rootFiles: files, getAst: (path: string) => astMap.get(path) };
};

const getAst = (p: string, includeParent: boolean): ts.Node => {
    const file = readFileSync(p, {
        encoding: 'utf-8',
        flag: 'r'
    });

    return ts.createSourceFile(p, file, ts.ScriptTarget.ES2015, includeParent);
};

function* getRefModules(ast: ts.Node, compilerOptions: ts.CompilerOptions) {
    const sourceFile = ast as SourceFile;
    const sourceFileName = sourceFile.fileName;
    for (const statement of sourceFile.statements) {
        if (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) {
            let modSpec = (<ImportDeclaration>statement).moduleSpecifier as ts.StringLiteral;
            const resolution = ts.resolveModuleName(modSpec.text, sourceFileName, compilerOptions, {
                fileExists: ts.sys.fileExists,
                readFile: ts.sys.readFile,
            }, cache);

            yield [modSpec.text, resolution.resolvedModule?.resolvedFileName];
        }
    }

    return [];
};