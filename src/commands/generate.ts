import fg from 'fast-glob'
import { Command } from 'commander';
import path from 'path';
import { CompilerOptions } from 'typescript';
import { generateSchemas, OpenApiOptions } from '../generator'

const generateOpenApi = (cwd: string, config: OpenApiOptions, compilerOptions: CompilerOptions = {}) => {
    generateSchemas({
        cwd,
        openApi: config ? {
            project: config.project,
            base: JSON.stringify(config.base),
            paths: fg.sync(config.paths, {
                absolute: true,
                cwd
            }),
            output: config.output
        } : undefined
    });
};

export default new Command('generate')
    .description('Generate one or more schemas')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, parentOptions?.config));
        generateOpenApi(parentOptions?.cwd, config.openApi, config.compilerOptions);
    });