import { BodyParam, Header, Path, QueryParam, Response, RouteParam } from '../../../src';
import admin, { CreateUserRequest, User, UserPatch } from '../dtos';

interface Reply<T> {
    send: (responseType: T) => void;
}

const router = {
    get: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    delete: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    patch: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    post: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    put: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    }
};

router.get("", {}, Path(async (request: { lat: QueryParam<number, false>, long: QueryParam<number, false>; headers: { user: Header<User, true, "v1">; }; }, reply: any): Promise<void> => {
    let success = {} as User;

    let response = Response(success, {
        statusCode: 200,
        namespace: "v1",
        description: "Who am I",
        example: "User"
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/user',
    tags: ['space'],
}));

// router.patch("", {}, Path(async (request: { id: RouteParam<string, true>; date: QueryParam<string, false>; }, reply: any): Promise<void> => {
//     let admin = new AdminUser();

//     let response = Response(admin, {
//         statusCode: 202,
//         description: "a modified admin user"
//     });

//     reply.send(response);
// }, {
//     method: 'PATCH',
//     path: '/user/{id}',
// }));

// router.get("", {}, Path(async (request: { id: RouteParam<string, true>; }, reply: any): Promise<void> => {
//     let response = Response(new AdminUser(), {
//         statusCode: 200,
//         description: "a specific admin user"
//     });

//     reply.send(response);
// }, {
//     method: 'GET',
//     path: '/user/{id}',
// }));

// router.delete("", {}, Path(async (request: { id: RouteParam<string, true>; }, reply: any): Promise<void> => {
//     let response = Response(null, {
//         statusCode: 204,
//         description: "no content",
//         namespace: "v1",
//         example: "NoContent"
//     });

//     reply.send(response);
// }, {
//     method: 'DELETE',
//     path: '/user/{id}',
//     tags: ["Admin", "Users"],
// }));

// router.post("", {}, Path(async (request: { user: BodyParam<CreateUserRequest, true, "v1">; }, reply: any): Promise<void> => {
//     let response = Response(<User>{}, {
//         statusCode: 201,
//         description: "Create a new user",
//         namespace: "v1"
//     });

//     reply.send(response);
// }, {
//     method: 'POST',
//     path: '/user',
//     tags: ["Admin"]
// }));

// router.put("", {}, Path(async (request: { tomato: BodyParam<UserPatch, false, "v1">; }, reply: any): Promise<void> => {
//     let response = Response({ first: 'John', title: 'Sr. Manager' }, {
//         statusCode: 202,
//         description: "Patched User",
//         namespace: "v1"
//     });

//     reply.send(response);
// }, {
//     method: 'PUT',
//     path: '/user',
//     tags: ["User"]
// }));