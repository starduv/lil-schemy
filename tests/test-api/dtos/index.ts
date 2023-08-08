import { Header, QueryParam, RouteParam } from '../../../src';

export { CreateUserRequest, default as UserPatch } from './requests';

export interface Account {
    number: string;
}

export interface User {
    name: string;
}

class AdminUser {
    permissions!: string[];
    name!: string;
}

export interface GetUserRequest { lat: QueryParam<number, false>, long: QueryParam<number, false>; headers: { user: Header<User, true, "v1">; }; }

export interface UserPatchRequest { id: RouteParam<string, true>; date: QueryParam<string, false, undefined, "date">; }

export default AdminUser