import { Header, QueryParam } from '../../../src';

export { CreateUserRequest, default as UserPatch } from './requests';

export interface User {
    name: string;
}

export default class AdminUser implements User {
    permissions!: string[];
    name!: string;
}

export interface GetUserRequest { lat: QueryParam<number, false>, long: QueryParam<number, false>; headers: { user: Header<User, true, "v1">; }; }

