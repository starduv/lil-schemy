export { default as UserPatch, CreateUserRequest } from './requests'

export interface User {
    name: string;
}

export default class AdminUser implements User {
    permissions!: string[];
    name!: string;
}

