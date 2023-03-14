import { sync as fg } from 'fast-glob';
import { readFileSync } from 'fs';
import ts, { ImportDeclaration, SourceFile } from 'typescript';

export interface TypeShiftContext {
    asts: { [path: string]: ts.Node },
    moduleNames: { [alias: string]: string }
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

    console.log(cwd)
    const moduleNames: { [alias: string]: string } = {}
    const astMap: { [path: string]: ts.Node } = {};
    const files = fg(globs, {
        absolute: true,
        cwd
    });
    const fileNames: Array<RefPaths> = files.map(fullPath => [undefined, fullPath]);

    while (fileNames.length) {
        const [rel, full] = fileNames.pop() as [string | null, string];

        if (rel) moduleNames[rel] = full;

        if (!astMap[full]) {
            astMap[full] = getAst(full);
        }

        for (const paths of getRefModules(astMap[full] as ts.Node, compilerOptions)) {
            if (paths[0] && paths[1]) {
                fileNames.push(paths as RefPaths);
            }
        }
    }

    console.debug("generated ast(s) for ", Object.keys(astMap));
    // writeFileSync("/home/captainrdubb/dev/serde_strong/types.json", JSON.stringify(astMap));

    return { rootFiles: files, asts: astMap, moduleNames };
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