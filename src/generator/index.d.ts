import { OpenAPIV3 } from 'openapi-types';

export interface LilSchemyOptions {
    cwd: string;
    openApi?: OpenApiOptions;
}

interface OpenApiOptions {
    base: Omit<OpenAPIV3.Document, "paths"> & {
        openapi: "3.0.3";
        paths?: OpenAPIV3.PathsObject;
    },
    output?: string;
    entry: string[];
}

export interface SchemasResult {
    openApi: {
        schema?: string;
        filepath?: string;
    };
}

interface GenerateSchemaArgs extends Omit<LilSchemyOptions, "cwd"> {
    // getAst: (reference: string, moduleFileName: string) => string | undefined;
    openApi?: Omit<OpenApiOptions, "base"> & {
        base: string;
    };
}

export function generateSchemas(options: GenerateSchemaArgs): SchemasResult;
