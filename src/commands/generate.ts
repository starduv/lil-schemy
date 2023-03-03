import { Command } from 'commander';
import path from 'path';
import { generateSchemas, OpenApiOptions } from '../generator'
import { getContext } from '../utils';

const generateOpenApi = (cwd: string, config: OpenApiOptions, project: string) => {
    const context = getContext(cwd, config.paths, {
        project
    })

    generateSchemas({
        asts: context.asts,
        modules: JSON.stringify(context.moduleNames),
        openApi: {
            base: JSON.stringify(config.base),
            paths: context.rootFiles,
            output: config.output
        }
    });
};

export default new Command('generate')
    .description('Generate one or more schemas')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, parentOptions?.config));
        generateOpenApi(parentOptions?.cwd, config.openApi, config.project);
    });