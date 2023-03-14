import { Command, Option } from 'commander';
import path from 'path';
import { generateSchemas, OpenApiOptions, TypeShiftOptions } from '../generator'
import { getContext } from '../utils';

const generateOpenApi = (config: TypeShiftOptions) => {
    const { openApi, project } = config;

    if (openApi) {
        const context = getContext(config.cwd, openApi.paths, {
            project
        })

        generateSchemas({
            asts: JSON.stringify(context.asts),
            modules: JSON.stringify(context.moduleNames),
            openApi: {
                base: JSON.stringify(openApi.base),
                paths: context.rootFiles,
                output: openApi.output
            }
        });
    }

};

export default new Command('generate')
    .description('Generate one or more schemas')
    .option('-c, --config <config>', 'configuration module', 'typeshift')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, command.getOptionValue('config')));
        generateOpenApi(config);
    });