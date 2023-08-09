import { RouteParam } from "../../../src";

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
        kind: AnimalKind
    }
}

enum AnimalKind {
    Dog = "dog",
    Cat = "cat",
    Bird = "bird"
}
