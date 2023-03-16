import { Header, QueryParam, RouteParam } from '../../../src';

export { CreateUserRequest, default as UserPatch } from './requests';

export interface User {
    name: string;
}

export default class AdminUser implements User {
    permissions!: string[];
    name!: string;
}

type ID = RouteParam<string, true>;
type Date = QueryParam<string, false, undefined, "date">;

export interface GetUserRequest { lat: QueryParam<number, false>, long: QueryParam<number, false>; headers: { user: Header<User, true, "v1">; }; }

export interface UserPatchRequest { id: ID; date: Date; }

