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

    it('sets OpenApi version', () => {
        expect(schema.openapi).to.eq("3.1.0");
    });

    it('generates schemas', () => {
        expect(schema.components?.schemas).to.deep.equalInAnyOrder({
            Account: {
                properties: {
                    number: {
                        type: "string"
                    }
                },
                type: "object"
            },
            AdminUser: {
                properties: {
                    name: {
                        type: "string"
                    },
                    permissions: {
                        items: {
                            type: "string"
                        },
                        type: "array"
                    }
                },
                type: "object"
            },
            Registration: {
                properties: {
                    date: {
                        type: "string"
                    }
                },
                type: "object"
            },
            CreateUserRequest: {
                properties: {
                    name: {
                        type: "string"
                    }
                },
                type: "object"
            },
            AnimalKind: {
                enum: [
                    "dog",
                    "cat",
                    "bird"
                ],
                type: "string"
            },
            User: {
                properties: {
                    name: {
                        type: "string"
                    }
                },
                type: "object"
            },
            AnimalUpdate: {
                allOf: [
                    {
                        $ref: "#/components/schemas/Registered"
                    },
                    {
                        properties: {
                            name: {
                                type: "string"
                            },
                            mood: {
                                $ref: "#/components/schemas/AnimalMood"
                            }
                        },
                        type: "object"
                    }
                ]
            },
            Registered: {
                properties: {
                    serialNumber: {
                        type: "string"
                    },
                    record: {
                        $ref: "#/components/schemas/Registration"
                    }
                },
                type: "object"
            },
            UserPatch: {
                type: "object"
            },
            AdjacentLicense: {
                $ref: "#/components/schemas/AnimalLicense"
            },
            AnimalLicense: {
                properties: {
                    exp: {
                        type: "string"
                    },
                    state: {
                        type: "string"
                    },
                    adjacents: {
                        items: {
                            $ref: "#/components/schemas/AdjacentLicense"
                        },
                        type: "array"
                    }
                },
                type: "object"
            },
            AnimalMood: {
                anyOf: [
                    {
                        properties: {
                            ambivalence: {
                                type: "number"
                            }
                        },
                        type: "object"
                    },
                    {
                        enum: [
                            "happy",
                            "sad",
                            "angry"
                        ]
                    }
                ]
            }
        });
    });

    it('generates paths', () => {
        expect(schema.paths).to.deep.equal({
            "/animals/{id}/unregister": {
                post: {
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
                            description: "A specific animal",
                            content: {
                                "application/json": {
                                    schema: {
                                        properties: {
                                            shots: {
                                                items: {
                                                    type: "string"
                                                },
                                                type: "array"
                                            },
                                            name: {
                                                type: "string"
                                            }
                                        },
                                        type: "object"
                                    }
                                }
                            }
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
                            "multipart/form-data": {
                                schema: {
                                    properties: {
                                        freindliness: {
                                            type: "number"
                                        },
                                        name: {
                                            type: "string"
                                        },
                                        photo: {
                                            type: "string",
                                            format: "binary"
                                        }
                                    },
                                    type: "object"
                                }
                            }
                        },
                        required: true
                    },
                    responses: {
                        200: {
                            description: "Status of animal registration",
                            content: {
                                "application/xml": {
                                    schema: {
                                        properties: {
                                            status: {
                                                type: "string"
                                            }
                                        },
                                        type: "object"
                                    }
                                }
                            }
                        },
                        404: {
                            description: "animal not found",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            }
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
                            name: "id",
                            in: "path",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string"
                                    }
                                }
                            },
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
                    tags: [
                        "Account"
                    ]
                }
            },
            "/animals": {
                get: {
                    parameters: [
                        {
                            name: "kind",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AnimalKind"
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "List animals of a specific kind",
                            content: {
                                "application/json": {
                                    schema: {
                                        items: {
                                            type: "string"
                                        },
                                        type: "array"
                                    }
                                }
                            }
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/user/{id}": {
                get: {
                    parameters: [
                        {
                            name: "id",
                            in: "path",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string"
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
                delete: {
                    parameters: [
                        {
                            name: "id",
                            in: "path",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string"
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
                                    schema: {},
                                    example: {
                                        $ref: "#/components/examples/NoContent"
                                    }
                                }
                            }
                        }
                    },
                    tags: [
                        "Admin",
                        "Users"
                    ]
                },
                patch: {
                    parameters: [
                        {
                            name: "id",
                            in: "path",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "string"
                                    }
                                }
                            },
                            required: true
                        },
                        {
                            name: "date",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        format: "date",
                                        type: "string"
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
                                        $ref: "#/components/schemas/AdminUser"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/animals/{id}": {
                get: {
                    parameters: [
                        {
                            name: "id",
                            in: "path",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "A specific animal",
                            content: {
                                "application/json": {
                                    schema: {
                                        properties: {
                                            shots: {
                                                items: {
                                                    type: "string"
                                                },
                                                type: "array"
                                            },
                                            name: {
                                                type: "string"
                                            }
                                        },
                                        type: "object"
                                    }
                                }
                            }
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                },
                post: {
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
                            description: "A specific animal",
                            content: {
                                "application/json": {
                                    schema: {
                                        properties: {
                                            shots: {
                                                items: {
                                                    type: "string"
                                                },
                                                type: "array"
                                            },
                                            name: {
                                                type: "string"
                                            }
                                        },
                                        type: "object"
                                    }
                                }
                            }
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
                            description: "A specific animal license",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AnimalLicense"
                                    }
                                }
                            }
                        }
                    },
                    tags: [
                        "Animals"
                    ]
                }
            },
            "/user": {
                get: {
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
                                        $ref: "#/components/schemas/User"
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "Who am I",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/User"
                                    },
                                    example: {
                                        $ref: "#/components/examples/User"
                                    }
                                }
                            }
                        }
                    },
                    tags: [
                        "Users"
                    ]
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
                    tags: [
                        "User"
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
                    tags: [
                        "Admin"
                    ]
                }
            }
        });
    });
});