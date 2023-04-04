import { Command } from 'commander';
import path from 'path';
import { generateSchemas, TypeShiftOptions } from '../generator';
import { getRootFiles } from '../utils';

export const generateOpenApi = (cwd: string, config: TypeShiftOptions) => {
    if (config?.openApi) {
        const { openApi, project } = config;

        const files = getRootFiles(cwd, openApi.paths);

        console.debug("Searching for api paths in files %o", files);

        const result = generateSchemas({
            // getAst: getAst(cwd, { project }),
            openApi: {
                base: JSON.stringify(openApi.base),
                paths: files,
                output: openApi.output || undefined
            }
        });

        if (result.openApi?.schema) {
            console.info(result.openApi.schema);
        } else if (result.openApi?.filepath) {
            console.info("OpenApi schema written to %s", result.openApi.filepath);
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