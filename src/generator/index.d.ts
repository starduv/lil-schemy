import { OpenAPIV3 } from 'openapi-types';

export interface LilSchemyOptions {
    openApi?: OpenApiOptions;
}

interface OpenApiOptions {
    base: Omit<OpenAPIV3.Document, "openapi"> & {
        openapi: "3.0.3";
    },
    output?: string;
    entry: string[];
}

export interface LilSchemyResult {
    openApi: OpenApiResult;
}

export interface OpenApiResult {
    schema?: string;
    filepath?: string;
}

interface GenerateSchemaArgs extends Omit<LilSchemyOptions, "cwd"> {
    // getAst: (reference: string, moduleFileName: string) => string | undefined;
    openApi?: Omit<OpenApiOptions, "base"> & {
        base: string;
    };
}

export function generateSchemas(options: GenerateSchemaArgs): LilSchemyResult;
