import { sync as fg } from 'fast-glob';
import { appendFileSync, readFileSync, writeFileSync } from 'fs';
import ts, { ImportDeclaration, SourceFile } from 'typescript';

export interface TypeShiftContext {
    asts: string,
    rootFiles: string[],
}

type RefPaths = [string | undefined, string | undefined];

let cache: ts.ModuleResolutionCache;

export const getContext = (cwd: string, globs: string[], compilerOptions: ts.CompilerOptions): TypeShiftContext => {
    if (!cache) {
        cache = ts.createModuleResolutionCache(
            cwd,
            path => path,
            compilerOptions
        );
    }

    const astMap: { [path: string]: ts.Node } = {};
    const masterCache: Map<string, ts.Node> = new Map();
    const files = fg(globs, {
        absolute: true,
        cwd
    });

    const fileNames: Array<RefPaths> = files.map(fullPath => [undefined, fullPath]);

    while (fileNames.length) {
        const [rel, full] = fileNames.pop() as [string | null, string];
        if (!masterCache.has(full)) {
            masterCache.set(full, getAst(full));
        }

        const ast = masterCache.get(full);
        astMap[rel || full] = ast as ts.Node;

        for (const paths of getRefModules(ast as ts.Node, compilerOptions)) {
            if (paths[0] && paths[1]) {
                fileNames.push(paths as RefPaths);
            }
        }
    }

    console.debug("generated ast(s) for ", Object.keys(astMap));
    // writeFileSync("/home/captainrdubb/dev/serde_strong/types.json", JSON.stringify(astMap));

    return { rootFiles: files, asts: JSON.stringify(astMap) };
};

const getAst = (p: string): ts.Node => {
    const file = readFileSync(p, {
        encoding: 'utf-8',
        flag: 'r'
    });

    return ts.createSourceFile(p, file, ts.ScriptTarget.ES2015, false);
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