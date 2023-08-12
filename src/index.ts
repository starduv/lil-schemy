export { generateOpenApi } from './commands/generate';
export { getRootFiles } from './utils';

type OptionalString = string | undefined;
type OptionalArray<T> = T[] | undefined;
type NumberFormat = "int32" | "int64" | "float" | "double";
type F = StringFormat | NumberFormat | undefined;
type StringFormat = "date-time" |
    "time" |
    "date" |
    "duration" |
    "email" |
    "idn-email" |
    "hostname" |
    "idn-hostname" |
    "ipv4" |
    "ipv6" |
    "uuid" |
    "uri" |
    "uri-reference" |
    "uri-template" |
    "json-pointer" |
    "relative-json-pointer" |
    "regex" |
    "iri" |
    "iri-reference" |
    undefined;


interface BodyParamArgs { required: boolean }


export type OperationMethod = 'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE';

export interface PathItemOptions {
    method: OperationMethod;
    path: string;
    tags?: string[];
}
export function LilPath<Func>(fn: Func, options: PathItemOptions | null = null) {
    return fn;
}

export interface ResponseOptions {
    description?: string;
    example?: string;
    statusCode: number;
}
export function LilResponse<ResponseType>(response: ResponseType, options: ResponseOptions) {
    return response;
}

export type LilBodyParam<Param, Required extends boolean = true> = Param;

export type LilHeader<Param, Required extends boolean = true, Format extends F = undefined> = Param;

export type LilQueryParam<Param, Required extends boolean = false, Format extends F = undefined> = Param;

export type LilRouteParam<Param, Required extends true = true, Format extends F = undefined> = Param;

// export type LilPath<TParams extends LilPathParamArgs> = TParams;
// export type LilResponse<ResponseType, StatusCode extends number, Description extends OptionalString, Example extends OptionalString> = ResponseType;
// type LilPathParamArgs<QueryParam = undefined, BodyParam = undefined, RouteParam = undefined, Header = undefined, Response = undefined> = LilQueryParam<QueryParam> | LilBodyParam<BodyParam> | LilHeader<Header, any> | LilQueryParam<QueryParam, any> | LilRouteParam<any, any> | LilResponse<any, any, any, any>;
// type LilPathParams<LilPathParamArgs> = Array<LilPathParamArgs>;