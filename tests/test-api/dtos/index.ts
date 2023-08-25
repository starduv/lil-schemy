import { LilHeader, LilQueryParam, LilRouteParam } from '../../../src';

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

export interface GetUserRequest { lat: LilQueryParam<number, false>, long: LilQueryParam<number, false>; headers: { user: LilHeader<User, true, "v1">; }; }

export interface UserPatchRequest { id: LilRouteParam<string, true>; date: LilQueryParam<string, false, undefined, "date">; }

export default AdminUser