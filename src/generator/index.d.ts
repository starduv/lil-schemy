interface OpenApiOptions {
    base: OpenAPIV3.Document & {
        openapi: "3.0.3";
    },
    output?: string;
    paths: string[];
}

export interface TypeShiftOptions {
    cwd?: string;
    asts: string;
    openApi?: OpenApiOptions;
}

export interface SchemasResult {
    openApi: {
        schema?: string;
        isFile: boolean;
        filepath?: string;
    };
}

interface GenerateSchemaArgs extends TypeShiftOptions {
    openApi?: Omit<OpenApiOptions, "base"> & {
        base: string;
    }
}

export function generateSchemas(options: GenerateSchemaArgs): SchemasResult;
