/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/todo.json`.
 */
export type Todo = {
  "address": "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF",
  "metadata": {
    "name": "todo",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addTask",
      "docs": [
        "Add a new task to the todo list",
        "Validates name and description lengths",
        "Checks for duplicate names and max tasks limit"
      ],
      "discriminator": [
        234,
        40,
        30,
        119,
        150,
        53,
        76,
        83
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true,
          "relations": [
            "todo"
          ]
        },
        {
          "name": "todo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  100,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "Initialize a new todo list for a user",
        "Creates a PDA account to store tasks"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "todo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  100,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "taskComplete",
      "docs": [
        "Mark a task as complete",
        "Validates task index and completion status"
      ],
      "discriminator": [
        31,
        106,
        40,
        228,
        86,
        20,
        152,
        8
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true,
          "relations": [
            "todo"
          ]
        },
        {
          "name": "todo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  100,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "taskIndex",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "todo",
      "discriminator": [
        137,
        179,
        206,
        68,
        34,
        36,
        131,
        54
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "nameTooLong",
      "msg": "The provided name should be 20 characters long maximum."
    },
    {
      "code": 6001,
      "name": "nameTooShort",
      "msg": "The provided name should be at least four characters long."
    },
    {
      "code": 6002,
      "name": "nameAlreadyExists",
      "msg": "The provided name already exists."
    },
    {
      "code": 6003,
      "name": "descriptionTooLong",
      "msg": "The provided description should be 180 characters long maximum."
    },
    {
      "code": 6004,
      "name": "taskNotFound",
      "msg": "The specified task was not found."
    },
    {
      "code": 6005,
      "name": "maxTasksReached",
      "msg": "The maximum number of tasks has been reached."
    },
    {
      "code": 6006,
      "name": "taskAlreadyCompleted",
      "msg": "This task is already marked as completed."
    },
    {
      "code": 6007,
      "name": "insufficientFunds",
      "msg": "Insufficient funds to create todo list."
    }
  ],
  "types": [
    {
      "name": "todo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "tasks",
            "type": {
              "vec": {
                "defined": {
                  "name": "todoItem"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "todoItem",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "completed",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
