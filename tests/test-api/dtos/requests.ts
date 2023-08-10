import { QueryParam, RouteParam } from "../../../src";

export default interface PatchUserRequest { }

export type CreateUserRequest = {
    name: string
}

interface RequestGenericInterface {
    Body?: unknown;
    Querystring?: unknown;
    Params?: unknown;
    Headers?: unknown;
}

export interface Request<T extends RequestGenericInterface = RequestGenericInterface> { }

export interface GetAccountRequest { id: RouteParam<string, true>; }

export interface AnimalsRequest {
    Querystring: {
        kind: QueryParam<AnimalKind, true>
    }
}

enum AnimalKind {
    Dog = "dog",
    Cat = "cat",
    Bird = "bird"
}
