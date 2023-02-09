type Ref = string;

export type OperationMethod = 'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE';

export interface PathItemOptions {
    method: OperationMethod;
    path: string;
    tags?: string[];
}

export interface ResponseOptions {
    description?: string;
    example?: string;
    namespace?: string;
    statusCode: number;
}

export function Path<Func>(fn: Func, options: PathItemOptions | null = null) {
    return fn;
}

export function Response<ResponseType>(response: ResponseType, options: ResponseOptions) {
    return response;
}

export type BodyParam<Param, Required extends boolean, Namespace extends string | undefined = undefined> = Param;
export type Header<Param, Required extends boolean, Namespace extends string | undefined = undefined> = Param;
export type QueryParam<Param, Required extends boolean, Namespace extends string | undefined = undefined> = Param;
export type RouteParam<Param, Required extends true, Namespace extends string | undefined = undefined> = Param;
