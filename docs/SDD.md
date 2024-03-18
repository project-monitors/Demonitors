# SDD

[toc]

## Programs

### miner

- vote account 可以创建deposit pool account
  - pool account 可以创建stake account和mint account
  - 以及对应的stake authority和mint authority
  - 
- 
- 根据场景可以生成多种SBT
  - 所有的SBT属于同一个Collection
- 



EventConfig
通过OracleConfig PDA

每个下注都是一个EventMarket
EventMarket分多种：

RawDataEventMarket
通过"raw-data", eventconfigPDA, started, finalize

PhaseEventMarket

每个人都可以InitialMarket，但是不能影响全局配置

Deposit
SBT
positionAccount
If first Create Mint 1 SBT For Him
Only 1

先就支持这一个




每天结算一次 每天开启3个Market 定时任务

Expired

Finalized


参与前提




Event Program

Finalizer client


Settle Program Mint for



在Market放置一个区块高度（用于获得当初oracle Data里的raw_data）

### monitor

Monitor programs承担“互联网景观”预言机链上部分的功能。

#### Accounts

预言机分为两个account，OracleConfig承载预言机的配置，OracleData承载预言机的数据。

##### OracleConfig

Oracle Config是一个PDA，seed由"oracle-config"和name构成。

```js
anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("oracle-config"),
        Buffer.from(name),
    ],
    program.programId
)

```

Oracle Config Account中存储预言机的配置。
OracleConfig Account数据结构：
- name: 名称，需小于31 bytes
- description: 描述，需小于200 bytes
- totalPhases: 作为预言机数据OracleData，总共由多少种状态。如果totalPhases是2，则phase可以取值0或1。这个和后续预测市场对于状态预判相关。
- authorityPubkeys: 有权限写OracleData的Pubkeys
- admin: 有权限写OracleConfig的Admin Pubkey
- bump: OracleConfig的PDA bump

```json
    {
      "name": "OracleConfig",
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
            "name": "totalPhases",
            "type": "u8"
          },
          {
            "name": "authorityPubkeys",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
```
##### OracleData

Oracle Data是一个PDA，seed由"oracle-data"和OracleConfig的Pubkey构成。

```js
anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("oracle-data"),
        config_pubkey.toBuffer(),
    ],
    program.programId
);
```

Oracle Data存储预言机的数据，与Oracle Config一一对应。
OracleData Account数据结构：
- config: 对应的Oracle Config Pubkey
- phase: 处于的状态，必须小于Oracle Config的totalPhases
- rawData: 原始数据，需要为正整数
- decimals: 原始数据的精度，和rawData拼成真实数据
- timestamp: 数据的时间戳，当下是根据上链成功时间由Program来计算
- bump: OracleData的PDA bump

```json
    {
      "name": "OracleData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "config",
            "type": "publicKey"
          },
          {
            "name": "phase",
            "type": "u8"
          },
          {
            "name": "rawData",
            "type": "u64"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
```

#### Instructions

##### initializeOracleConfig

初始化一个预言机配置:

```json
{
      "name": "initializeOracleConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authorityPubkey",
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "totalPhase",
          "type": "u8"
        }
      ]
    }
```

##### addAuthorityToOracleConfig

增加可以操作预言机Data的Pubkey。

```json
 {
      "name": "addAuthorityToOracleConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authorityPubkey",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
```

##### removeAuthorityFromOracleConfig

去除可以操作预言机Data的Pubkey。

```json
{
  "name": "removeAuthorityFromOracleConfig",
  "accounts": [
    {
      "name": "config",
      "isMut": true,
      "isSigner": false
    },
    {
      "name": "user",
      "isMut": true,
      "isSigner": true
    },
    {
      "name": "authorityPubkey",
      "isMut": false,
      "isSigner": false
    }
  ],
  "args": []
}
```

##### initializeOracleData

初始化OracleData Account。

```json
    {
      "name": "initializeOracleData",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
```

##### setOracleData

向OracleData Account写入数据。

```json
    {
      "name": "setOracleData",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "phase",
          "type": "u8"
        },
        {
          "name": "rawData",
          "type": "u64"
        },
        {
          "name": "decimals",
          "type": "u8"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    }
```

#### Events

##### SetOracleDataEvent

向OracleData Account写入数据后会释放的时间
- state: OracleData的Pubkey
- phaseChange: phase的变化，如果有变化会显示old值和new值
- rawDataChange: rawData的变化，如果有变化会显示old值和new值
- decimalsChange: decimals的变化，如果有变化会显示old值和new值
- timestampChange：timestamp的变化，如果有变化会显示old值和new值

```json
    {
      "name": "SetOracleDataEvent",
      "fields": [
        {
          "name": "state",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "phaseChange",
          "type": {
            "option": {
              "defined": "U8ValueChange"
            }
          },
          "index": false
        },
        {
          "name": "rawDataChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        },
        {
          "name": "decimalsChange",
          "type": {
            "option": {
              "defined": "U8ValueChange"
            }
          },
          "index": false
        },
        {
          "name": "timestampChange",
          "type": {
            "option": {
              "defined": "U64ValueChange"
            }
          },
          "index": false
        }
      ]
    }
```
