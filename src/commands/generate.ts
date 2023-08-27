import { Command } from 'commander';
import path from 'path';
import { generateSchemas, LilSchemyOptions as LilSchemyOptions, LilSchemyResult as LilSchemyResult } from '../generator';
import { getRootFiles } from '../utils';

export const generate = (cwd: string, options: LilSchemyOptions): LilSchemyResult => {
    const { openApi } = options;

    const files = getRootFiles(cwd, openApi?.entry ?? []);

    console.debug("Searching for api paths in files %o", files);

    const result = generateSchemas({
        openApi: {
            base: JSON.stringify(openApi?.base ?? {}),
            entry: files,
            output: openApi?.output
        }
    });

    if (result.openApi?.filepath) {
        console.info("OpenApi schema written to %s", result.openApi.filepath);
    }

    return result;
};

export default new Command('generate')
    .description('Generate one or more schemas')
    .option('-c, --config <config>', 'configuration module', 'schemy-config')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, command.getOptionValue('config')));
        generate(parentOptions?.cwd, config.default ?? config);
    });