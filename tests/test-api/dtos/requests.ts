import { LilQueryParam, LilRouteParam, LilSub } from "../../../src";

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

export interface GetAccountRequest { id: LilRouteParam<string, true>; }

export interface AnimalsRequest {
    Querystring: {
        kind: LilQueryParam<AnimalKind, true>
    }
}

enum AnimalKind {
    Dog = "dog",
    Cat = "cat",
    Bird = "bird"
}

export type AnimalMood = "happy" | "sad" | "angry" | { ambivalence: number };

export interface AnimalUpdate extends Omit<Registered, "serialNumber"> {
    name: string;
    mood: AnimalMood;
}

interface Registered {
    serialNumber: string;
    record: Registration;
}

interface Registration {
    date: string;
}

export interface AnimalLicense {
    state: string;
    exp: LilSub<Date, string>;
    adjacents: Array<AdjacentLicense>;
}

export type AdjacentLicense = AnimalLicense;