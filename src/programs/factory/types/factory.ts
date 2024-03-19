export type Factory = {
  "version": "0.1.0",
  "name": "factory",
  "instructions": [
    {
      "name": "initializeGlobalConfig",
      "accounts": [
        {
          "name": "globalConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningAdminPubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "changeVisionMiningAdmin",
      "accounts": [
        {
          "name": "globalConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningAdminPubkey",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeMint",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK - create token metadata account manually"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "InitializeMintParams"
          }
        }
      ]
    },
    {
      "name": "mintTokens",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "MintTokensParams"
          }
        }
      ]
    },
    {
      "name": "visionMiningClaim",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningAdmin",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "VisionMiningClaimParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalConfig",
      "docs": [
        "The [GlobalConfig] account."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "The admin of the factory program."
            ],
            "type": "publicKey"
          },
          {
            "name": "visionMiningAdmin",
            "docs": [
              "The off-chain supervisor for vision mining."
            ],
            "type": "publicKey"
          },
          {
            "name": "visionMiningPda",
            "type": "publicKey"
          },
          {
            "name": "eventMiningPda",
            "type": "publicKey"
          },
          {
            "name": "stakeMiningPda",
            "type": "publicKey"
          },
          {
            "name": "globalConfigBump",
            "type": "u8"
          },
          {
            "name": "visionMiningBump",
            "type": "u8"
          },
          {
            "name": "eventMiningBump",
            "type": "u8"
          },
          {
            "name": "stakeMiningBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "mintConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "mintBump",
            "type": "u8"
          },
          {
            "name": "configBump",
            "type": "u8"
          },
          {
            "name": "metadataBump",
            "type": "u8"
          },
          {
            "name": "authorityBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "U64ValueChange",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "old",
            "type": "u64"
          },
          {
            "name": "new",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "InitializeMintParams",
      "docs": [
        "Parameters for initializing the governance token mint account"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "decimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "MintTokensParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VisionMiningClaimParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "validUntilTime",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BalanceChangeEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Mint"
          },
          {
            "name": "Transfer"
          },
          {
            "name": "Burn"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "BalanceChangeEvent",
      "fields": [
        {
          "name": "eventType",
          "type": {
            "defined": "BalanceChangeEventType"
          },
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "fromTokenAccount",
          "type": {
            "option": "publicKey"
          },
          "index": false
        },
        {
          "name": "fromChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        },
        {
          "name": "toTokenAccount",
          "type": {
            "option": "publicKey"
          },
          "index": false
        },
        {
          "name": "toChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "Unauthorized",
      "msg": "You are not authorized to perform this action."
    },
    {
      "code": 6001,
      "name": "ReInitialize",
      "msg": "The account has already been initialized."
    },
    {
      "code": 6002,
      "name": "UnInitialize",
      "msg": "The account has not been initialized."
    },
    {
      "code": 6003,
      "name": "InvalidArgument",
      "msg": "Argument is invalid."
    },
    {
      "code": 6004,
      "name": "InvalidProgramId",
      "msg": "Program ID is invalid."
    },
    {
      "code": 6005,
      "name": "UnexpectedAccount",
      "msg": "Unexpected Account."
    },
    {
      "code": 6006,
      "name": "Overflow",
      "msg": "An overflow occurs."
    },
    {
      "code": 6007,
      "name": "StringTooLong",
      "msg": "The string variable is too long."
    },
    {
      "code": 6008,
      "name": "TooManyAuthorities",
      "msg": "Authorities limit reached"
    },
    {
      "code": 6009,
      "name": "AuthorityNotFound",
      "msg": "Authority not found"
    },
    {
      "code": 6010,
      "name": "ConfigMismatched",
      "msg": "Oracle config mismatched"
    },
    {
      "code": 6011,
      "name": "MintExceedMaxSupply",
      "msg": "Minting exceeds max supply limit"
    },
    {
      "code": 6012,
      "name": "NotSufficientBalance",
      "msg": "The transfer_from account does not have sufficient balance"
    },
    {
      "code": 6013,
      "name": "TransactionTimeout",
      "msg": "The transaction is timeout"
    }
  ]
};

export const IDL: Factory = {
  "version": "0.1.0",
  "name": "factory",
  "instructions": [
    {
      "name": "initializeGlobalConfig",
      "accounts": [
        {
          "name": "globalConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningAdminPubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "changeVisionMiningAdmin",
      "accounts": [
        {
          "name": "globalConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningAdminPubkey",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeMint",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK - create token metadata account manually"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sysvarInstruction",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "InitializeMintParams"
          }
        }
      ]
    },
    {
      "name": "mintTokens",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "MintTokensParams"
          }
        }
      ]
    },
    {
      "name": "visionMiningClaim",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "visionMiningAdmin",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "globalConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "visionMiningTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "VisionMiningClaimParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalConfig",
      "docs": [
        "The [GlobalConfig] account."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "The admin of the factory program."
            ],
            "type": "publicKey"
          },
          {
            "name": "visionMiningAdmin",
            "docs": [
              "The off-chain supervisor for vision mining."
            ],
            "type": "publicKey"
          },
          {
            "name": "visionMiningPda",
            "type": "publicKey"
          },
          {
            "name": "eventMiningPda",
            "type": "publicKey"
          },
          {
            "name": "stakeMiningPda",
            "type": "publicKey"
          },
          {
            "name": "globalConfigBump",
            "type": "u8"
          },
          {
            "name": "visionMiningBump",
            "type": "u8"
          },
          {
            "name": "eventMiningBump",
            "type": "u8"
          },
          {
            "name": "stakeMiningBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "mintConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "metadata",
            "type": "publicKey"
          },
          {
            "name": "mintBump",
            "type": "u8"
          },
          {
            "name": "configBump",
            "type": "u8"
          },
          {
            "name": "metadataBump",
            "type": "u8"
          },
          {
            "name": "authorityBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "U64ValueChange",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "old",
            "type": "u64"
          },
          {
            "name": "new",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "InitializeMintParams",
      "docs": [
        "Parameters for initializing the governance token mint account"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "decimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "MintTokensParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VisionMiningClaimParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "validUntilTime",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BalanceChangeEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Mint"
          },
          {
            "name": "Transfer"
          },
          {
            "name": "Burn"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "BalanceChangeEvent",
      "fields": [
        {
          "name": "eventType",
          "type": {
            "defined": "BalanceChangeEventType"
          },
          "index": false
        },
        {
          "name": "mint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "fromTokenAccount",
          "type": {
            "option": "publicKey"
          },
          "index": false
        },
        {
          "name": "fromChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        },
        {
          "name": "toTokenAccount",
          "type": {
            "option": "publicKey"
          },
          "index": false
        },
        {
          "name": "toChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "Unauthorized",
      "msg": "You are not authorized to perform this action."
    },
    {
      "code": 6001,
      "name": "ReInitialize",
      "msg": "The account has already been initialized."
    },
    {
      "code": 6002,
      "name": "UnInitialize",
      "msg": "The account has not been initialized."
    },
    {
      "code": 6003,
      "name": "InvalidArgument",
      "msg": "Argument is invalid."
    },
    {
      "code": 6004,
      "name": "InvalidProgramId",
      "msg": "Program ID is invalid."
    },
    {
      "code": 6005,
      "name": "UnexpectedAccount",
      "msg": "Unexpected Account."
    },
    {
      "code": 6006,
      "name": "Overflow",
      "msg": "An overflow occurs."
    },
    {
      "code": 6007,
      "name": "StringTooLong",
      "msg": "The string variable is too long."
    },
    {
      "code": 6008,
      "name": "TooManyAuthorities",
      "msg": "Authorities limit reached"
    },
    {
      "code": 6009,
      "name": "AuthorityNotFound",
      "msg": "Authority not found"
    },
    {
      "code": 6010,
      "name": "ConfigMismatched",
      "msg": "Oracle config mismatched"
    },
    {
      "code": 6011,
      "name": "MintExceedMaxSupply",
      "msg": "Minting exceeds max supply limit"
    },
    {
      "code": 6012,
      "name": "NotSufficientBalance",
      "msg": "The transfer_from account does not have sufficient balance"
    },
    {
      "code": 6013,
      "name": "TransactionTimeout",
      "msg": "The transaction is timeout"
    }
  ]
};
