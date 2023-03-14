import { Command } from 'commander';
import path from 'path';
import { generateSchemas, TypeShiftOptions } from '../generator';
import { getContext } from '../utils';

const generateOpenApi = (cwd: string, config: TypeShiftOptions) => {
    if (config?.openApi) {
        const { openApi, project } = config;

        const context = getContext(cwd, openApi.paths, {
            project
        });

        const result = generateSchemas({
            asts: JSON.stringify(context.asts),
            modules: JSON.stringify(context.moduleNames),
            openApi: {
                base: JSON.stringify(openApi.base),
                paths: context.rootFiles,
                output: openApi.output || undefined
            }
        });

        if (result.openApi?.schema) {
            console.log(result.openApi.schema);
        }
    }
};

export default new Command('generate')
    .description('Generate one or more schemas')
    .option('-c, --config <config>', 'configuration module', 'typeshift')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, command.getOptionValue('config')));
        generateOpenApi(parentOptions?.cwd, config.default ?? config);
    });