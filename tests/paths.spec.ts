import { expect, use } from 'chai';
import deepEqual from 'deep-equal-in-any-order';
import { OpenAPIV3 } from 'openapi-types';
import { generateSchemas } from '../src/generator';
import { getRootFiles } from '../src/utils';

use(deepEqual);

describe('open api generator', () => {
    let schema: OpenAPIV3.Document;

    before(() => {
        const result = generateSchemas({
            openApi: {
                base: JSON.stringify({}),
                entry: getRootFiles(__dirname, ["test-api/routes/*.ts", "!test-api/routes/router.ts"]),
            }
        });

        schema = JSON.parse(result.openApi.schema || "");
    });

    it('generates schemas', () => {
        expect(schema.components?.schemas).to.deep.equalInAnyOrder({
            Animal: {
                type: "object",
                properties: {
                    name: {
                        type: "string"
                    },
                    shots: {
                        type: "array",
                        items: {
                            type: "string"
                        }
                    }
                },
            },
            AnimalKind: {
                enum: ["cat", "dog", "bird"],
                type: "string",
            },
            AnimalLicense: {
                properties: {
                    adjacents: {
                        items: {
                            $ref: "#/components/schemas/AdjacentLicense"
                        },
                        type: "array"
                    },
                    exp: {
                        type: "string"
                    },
                    state: {
                        type: "string"
                    }
                },
                type: "object"
            },
            AnimalMood: {
                anyOf: [
                    {
                        enum: [
                            "happy",
                            "sad",
                            "angry"
                        ]
                    },
                    {
                        type: 'object',
                        properties: {
                            ambivalence: {
                                type: "number"
                            }
                        },
                    }
                ]
            },
            AnimalUpdate: {
                allOf: [
                    {
                        $ref: "#/components/schemas/Registered",
                    },
                    {
                        type: "object",
                        properties: {
                            mood: {
                                $ref: "#/components/schemas/AnimalMood"
                            },
                            name: {
                                type: "string"
                            },
                        }
                    }
                ]
            },
            User: {
                type: "object",
                properties: {
                    name: {
                        type: "string"
                    }
                },
            },
            CreateUserRequest: {
                type: 'object',
                properties: {
                    name: {
                        type: 'string'
                    }
                },
            },
            Account: {
                type: 'object',
                properties: {
                    number: {
                        type: 'string'
                    }
                },
            },
            AdjacentLicense: {
                $ref: "#/components/schemas/AnimalLicense"
            },
            AdminUser: {
                type: 'object',
                properties: {
                    permissions: {
                        type: 'array',
                        items: {
                            type: 'string'
                        }
                    },
                    name: {
                        type: 'string'
                    }
                },
            },
            PostAnimalsIdRegisterStatus: {
                properties: {
                    status: {
                        type: "string"
                    }
                },
                type: "object"
            },
            Registered: {
                type: 'object',
                properties: {
                    serialNumber: {
                        type: 'string'
                    },
                    record: {
                        $ref: '#/components/schemas/Registration'
                    }
                },
            },
            Registration: {
                type: 'object',
                properties: {
                    date: {
                        type: 'string',
                    },
                },
            }
        });
    });

    it('generates paths', () => {
        expect(schema.paths).to.deep.equal({
            "/animals": {
                get: {
                    tags: ["Animals"],
                    responses: {
                        200: {
                            description: "List animals of a specific kind",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "array",
                                        items: {
                                            type: "string"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    parameters: [
                        {
                            in: "query",
                            name: "kind",
                            required: true,
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AnimalKind"
                                    }
                                },
                            }
                        }
                    ]
                },
            },
            "/animals/{id}": {
                get: {
                    tags: ["Animals"],
                    responses: {
                        200: {
                            description: "A specific animal",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/Animal"
                                    }
                                }
                            }
                        }
                    },
                    parameters: [
                        {
                            in: "path",
                            name: "id",
                            required: true,
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                },
                            }
                        }
                    ]
                },
                "post": {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/AnimalUpdate"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        200: {
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/Animal"
                                    }
                                }
                            },
                            description: "A specific animal"
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/animals/{id}/license": {
                get: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/AnimalUpdate"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        200: {
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AnimalLicense"
                                    }
                                }
                            },
                            description: "A specific animal license"
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/animals/{id}/register": {
                post: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    properties: {
                                        freindliness: {
                                            type: "number"
                                        },
                                        name: {
                                            type: "string"
                                        }
                                    },
                                    type: "object"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        200: {
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/PostAnimalsIdRegisterStatus"
                                    }
                                }
                            },
                            description: "Status of animal registration"
                        },
                        404: {
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            description: "animal not found"
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/animals/{id}/unregister": {
                post: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    "$ref": "#/components/schemas/AnimalUpdate"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        200: {
                            content: {
                                "application/json": {
                                    schema: {
                                        "$ref": "#/components/schemas/Animal"
                                    }
                                }
                            },
                            description: "A specific animal"
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/account": {
                get: {
                    parameters: [
                        {
                            content: {
                                "application/json": {
                                    "schema": {
                                        "type": "string"
                                    }
                                }
                            },
                            in: "path",
                            name: "id",
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "Get user account",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/Account"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["Account"]
                }
            },
            "/user": {
                get: {
                    responses: {
                        200: {
                            description: "Who am I",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/User",
                                    },
                                    example: {
                                        $ref: "#/components/examples/User"
                                    }
                                }
                            }
                        }
                    },
                    parameters: [
                        {
                            name: "lat",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            required: false
                        },
                        {
                            name: "long",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            required: false
                        },
                        {
                            name: "user",
                            in: "header",
                            content: {
                                "application/json": {
                                    schema: {
                                        "$ref": "#/components/schemas/User"
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    tags: [
                        "Users"
                    ]
                },
                post: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/CreateUserRequest"
                                }
                            }
                        },
                        required: true
                    },
                    responses: {
                        201: {
                            description: "Create a new user",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/User"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["Admin"]
                },
                put: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/UserPatch"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        202: {
                            description: "Updated User",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AdminUser"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["User"]
                }
            },
            "/user/{id}": {
                get: {
                    parameters: [
                        {
                            in: "path",
                            name: "id",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string",
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "a specific admin user",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AdminUser"
                                    }
                                }
                            }
                        }
                    }
                },
                patch: {
                    parameters: [
                        {
                            in: "path",
                            name: "id",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string",
                                    }
                                }
                            },
                            required: true
                        },
                        {
                            in: "query",
                            name: "date",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string",
                                        format: "date"
                                    }
                                }
                            },
                            required: false
                        }
                    ],
                    responses: {
                        202: {
                            description: "a modified admin user",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AdminUser",
                                    },
                                }
                            }
                        }
                    }
                },
                delete: {
                    parameters: [
                        {
                            in: "path",
                            name: "id",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string",
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    responses: {
                        204: {
                            description: "no content",
                            content: {
                                "application/json": {
                                    example: {
                                        $ref: "#/components/examples/NoContent"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["Admin", "Users"],
                }
            }
        });
    });
});