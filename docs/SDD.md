# SDD

[toc]


## stories

### 1.获取景观当前数据用于美术渲染：
- 读取OracleData(oracle_data) Account的raw_data数据
- 结合美术逻辑，进行渲染

### 2. 获取景观历史数据用于美术渲染：
- 读取OracleData(oracle_data) Account的previous_timestamp数据
- 结合timestamp来调用rpc查询历史数据
- 结合美术逻辑，进行渲染

#### backend api

```shell
curl --location --request GET 'https://api.monitare.xyz/api/v1/oracle_data/1711600000'
```
请求的最后填写时间戳，会返回离此时间戳最近的上一轮oracle_data数据。

返回结构：

- raw_data: 恐惧贪婪指数的值
- timestamp: 上一次的时间戳
- authoritative: 若为true，则数据来自于预言机合约；若为false，则是从第三方API获取的历史数据追加

```json
{
    "code": 0,
    "err": false,
    "msg": "OK",
    "data": {
        "raw_data": 80,
        "timestamp": 1711584012,
        "authoritative": true
    }
}
```


### 3. 渲染未来景观：
- 读取OracleData(oracle_data) Account的raw_data数据
- 根据open_ts和oracle_config来计算未来景观的EventMarket(event_market)
- 计算预测景观两个选项的EventPosition(event_position)
- 读取两个选项的event_position的amount
- 结合美术逻辑，进行渲染

### 4. 判断用户是否已经拥有主sbt
- 查询用户的sbt_mint账户是否存在

### 5. 铸造主sbt
- 参考"Should: I. Create SBT"构造第一个instruction
- 参考"Should: II. Mint SBT"构造第二个instruction
- 如果要将这两个instructions放入一个transaction的话，需要调高ComputeUnits的限额，并且调高限额的instruction**必须**得是这个transaction中的第一个instruction
- 铸造会有事件SBTMintEvent

```ts
const modifyComputeUnits = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
    units: 1000000
});

const addPriorityFee = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 10
});

let transaction = new anchor.web3.Transaction()
    .add(modifyComputeUnits)
    .add(addPriorityFee);
```

### 6. 判断用户的主SBT是否已经用于下注
- 通过主SBT的mint(sbt_mint)来查询Marker(marker)
- 如果marker的indicate不为零则已经使用了
- marker的event_market指向了用户在哪个event里下注

### 7. 判断当前的event_market里用户是否下注
- 通过EventMarket(event_market)和user_pubkey来计算UserPosition(user_position)
- 如果user_position存在，则已经下注
- 根据user_position可以找到marker，通过marker的indicate可以找到用户下注在哪个选项

### 8. 判断当前的event_market是否可以下注
- event_market的is_opened为true
- event_market的expiry_ts大于now
- event_market的result为0
- 用户的主SBT没有用于下注
- 当前的event_market用户没有下注

### 9. 下注
- 参考"Should: choose"
- indicate为下注的选项
- 下注会有事件ChooseEvent

### 10. 撤注
- 参考"Should: withdraw"
- 用户必须在当前的event_market中有下注
- indicate为用户曾经下注的选项，必须传入这个值的原因，是因为合约验证计算pda

### 11. 判断当前的event_market是否可以领取
- 当前的event_market的result不为0
- 用户已经在这个event_market下注

### 12. 领取
- 参考"Should: claim"的后半部分，mintTokens只是为了给eventMiningTokenAccount Mint $MONI, 前端请无视
- indicate为用户曾经下注的选项，必须传入这个值的原因，是因为合约验证计算pda
- 如果前端预判断account有奖，则需要需要调高ComputeUnits的限额，并且调高限额的instruction**必须**得是这个transaction中的第一个instruction
- 这个instruction非常复杂，里面的account说明和pda生成，请参考下面的Accounts对照表

### 13. 视觉挖矿

- 同前

## Program

### Addresses

#### devnet

- Monitor Program: 12YQKMkv1xZ1B4gwVMiGTcYvY1z6TpdFvyAWyjhuC63c
- Factory Program: 36KZHRWMKbGNsMZ2jMVuRbnMUrjzRr8kmjHyPJ9ipvFW
- OracleConfig Account: A2qYPWjeWXrcm2vwXEy11vigwyrN4zrmZbtzxR1MM8A5
- OracleData Account: Aj8wNCAbySPpDNxzFxrfqvBmJEqQ8CgLQTK7fGNNwm1d

### Accounts 

依照字母表顺序

#### associated_token_program

- SPL的Associated Token Account的program
- 目的是一个pda标准，由user_key和mint_key来唯一计算出一个token account pubkey

**pda**

无，它是一个Program

```ts
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
```

#### authority

- authority都是关联到mint的铸造权限
- 根据不同的instructions，可能有两类authority
  - 一类是mint FT token的，**这部分的指令前端不会用到，可以忽略**
  - 另一类是各类SBT mint的authority，它的pda生成逻辑关联到collection_mint

**pda**

```ts
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from(authority),
        mint_key.toBuffer(), // 对应sbt，这里需要填写collection_mint_pubkey
    ],
    program.programId
);
```

#### collection_mint

- 根据Metaplex的定义，collection是一类特殊的NFT，它用来框范一系列NFT的mint，让它们可以属于一个系列
- collection属于NFT，所以它会有mint账户，metadata账户和masterEdition账户
- 对于前端来说，只需要关心collection_mint就好
- collection_mint会pda衍生关乎所有SBT铸造权限的authority

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from(collection),
    ],
    program.programId
);

```

#### EventConfig(event_config)

- event预测市场的配置信息
- 和oracle_config、oracle_data一一对应

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("event"),
        oracle_config_pubkey.toBuffer(), // oracle_config的pubkey
    ],
    program.programId
);
```

#### EventMarket(event_market)

- event预测市场的实例
- 有event_config和开启时间戳（每天UTC0点时间）来生成的pda

**struct**

因为对于前端开发，这个结构可能有用，所以一并提供。

说明：

- openTs：开始时间
- closeTs：结束时间
- expiryTs： 禁止投票时间，目前同closeTs
- option：允许的投注的选项。例如该值为2，则可以投注1或者2。0不能用于投注
- result：最终的结果，改值等同于option中的某种选项。0意味着没有公布结果
- isOpened：是否可以下注

```json
    {
      "name": "eventMarket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "eventConfig",
            "type": "publicKey"
          },
          {
            "name": "openRawData",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "openPhase",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "option",
            "type": "u8"
          },
          {
            "name": "result",
            "type": "u8"
          },
          {
            "name": "prize",
            "type": "u64"
          },
          {
            "name": "openSlot",
            "type": "u64"
          },
          {
            "name": "openTs",
            "type": "u64"
          },
          {
            "name": "closeTs",
            "type": "u64"
          },
          {
            "name": "expiryTs",
            "type": "u64"
          },
          {
            "name": "isOpened",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
```

**pda**

```ts
const timestamp = new anchor.BN(timestamp_number)
const timestampBuffer= timestamp.toBuffer('be', 8);
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("event_market"),
        oracle_config_pubkey.toBuffer(), // !!!!oracle_config的pubkey!!!
        timestampBuffer
    ],
    program.programId
);
```

#### event_mining_pda

- 负责预测市场挖矿的转账发起

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ft_event_mining_pda"),
    ],
    program.programId // factory program id
);
```

#### event_mining_token_account

- event_mining_pda在mint($MONI)下的token account

**pda**

```ts
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

const sbt_mint_token_account = await getAssociatedTokenAddress(
    mint_pubkey, event_mining_pda_pubkey, true,
    TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
```

#### EventPosition（event_position）

- event_position是用来显示某个Market中，各个选项的下注情况

**struct**

因为对于前端开发，这个结构可能有用，所以一并提供。

说明：

- amount：下注数量

```json
    {
      "name": "eventPosition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "positionType",
            "type": {
              "defined": "PositionType"
            }
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
```

**pda**

```ts
const option_bn = new anchor.BN(option);
const option_buffer = option_bn.toBuffer("be", 1)
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("event_position"),
        event_market_pubkey.toBuffer(),
        option_buffer
    ],
    program.programId
);
```

#### event_sbt_edition

- 基于Metaplex对于edition的定义，edition是由master_edition print出来的一个副本
- event_sbt_edition它是event_sbt_master_edition的一个副本
- event_sbt_edition_mint对应的edition
- 它是一个Metaplex的Edition

**pda**

```ts
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        event_sbt_edition_mint.toBuffer(),
        Buffer.from("edition")
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
);
```

#### event_sbt_edition_metadata

- event_sbt_edition_mint对应的metadata
- 它是一个Metaplex的Metadata

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        event_sbt_edition_mint.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
);
```

#### event_sbt_edition_mint

- 基于Metaplex对于edition的定义，edition是由master_edition print出来的一个副本
- 每个用户要获得自己的event_sbt_edition，会有独立的mint
- 它是一个Token 2022的Mint，增加了non_transferable的Extension

**pda**

```ts
const option_BN = new anchor.BN(option);
const option_buffer = option_BN.toBuffer('be', 1);
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("sbt_mint"),
        event_config.toBuffer(),
        option_buffer,
        user_pubkey.toBuffer(),
    ],
    program.programId
);
```

#### event_sbt_edition_pda

- 它是一个在print edition时，必须会使用的一个Metaplex账户
- 它和Edition的铸造数相关，具体内部的机理本文档不做表述
- 它和EDITION_MARKER_BIT_SIZE常量相关，需要关注pda的生成

**pda**

```ts
const EDITION_MARKER_BIT_SIZE = 248;
let editionNumber = new anchor.BN(Math.floor(edition/EDITION_MARKER_BIT_SIZE));
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        master_edition_mint.toBuffer(),
        Buffer.from("edition"),
        Buffer.from(editionNumber.toString())
    ], MPL_TOKEN_METADATA_PROGRAM_ID
)
```

#### event_sbt_edition_token_account

- 它是event_sbt_edition_mint对应user的token account

**pda**

```ts
const event_sbt_edition_token_account = await getAssociatedTokenAddress(
    event_sbt_edition_mint, user_pubkey, true,
    TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

```

#### event_sbt_master_edition

- 从event_config出发，它支持几种选项，就会有几个event_sbt_master_edition
- event_sbt_master_edition是NFT，它有mint, metadata 和master_edition账号
- 它是Metaplex的MasterEdition

**pda**

```ts
        const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        event_sbt_master_edition_mint.toBuffer(),
        Buffer.from("edition")
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
);
```


#### event_sbt_master_edition_metadata

- 从event_config出发，它支持几种选项，就会有几个event_sbt_master_edition
- event_sbt_master_edition是NFT，它有mint, metadata 和master_edition账号
- 它是Metaplex的Metadata

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        event_sbt_master_edition_mint.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
);
```


#### event_sbt_master_edition_mint

- 从event_config出发，它支持几种选项，就会有几个event_sbt_master_edition
- event_sbt_master_edition是NFT，它有mint, metadata 和master_edition账号
- event_sbt_master_edition是event_sbt_edition的母板，它通过print来产生event_sbt_edition
- 它是Token 2022的Mint，支持了non_transferable的Extension

**pda**

```ts
const option_BN = new anchor.BN(option);
const option_buffer = option_BN.toBuffer('be', 1);
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("sbt_mint"),
        event_config.toBuffer(),
        option_buffer
    ],
    program.programId
);
```

#### event_sbt_master_edition_token_account

- 这是event_sbt_master_edition_mint对应authority用户的token account
- 意味着event_sbt_master_edition被mint给了authority
- 它是Token 2022的Token Account

**pda**

```ts
const event_sbt_master_edition_token_account = await getAssociatedTokenAddress(
    event_sbt_master_edition_mint, authority, true,
    TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
```

#### GlobalConfig(global_config)

- GlobalConfig是Program的全局配置，存放了各类的admin信息：
  - governor：负责所有event相关的生成
  - vision_mining_admin：负责视觉挖矿的管理员
  - vision_mining_pda，event_mining_pda：负责发起视觉挖矿和预测市场挖矿的transfer

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("global_config"),
    ],
    program.programId // factory program id
);
```

#### governor

- 是Event中，创建event_config，event_market以及event相关SBT的多签管理员
- 在未来是代表一个社区多签身份

**pda**

无，此Account是一个System Account


#### Marker(marker)

- 对应每个主SBT的mint账户（sbt_mint），都有一个marker，来记录这个sbt_mint的投注情况

**struct**

因为对于前端开发，这个结构可能有用，所以一并提供。

说明：

- indicate：这个sbt_mint下注了哪个选项，如果是0，则没有用于下注
- eventMarket：这个sbt_mint下注在哪个市场

```json
{
  "name": "marker",
  "type": {
    "kind": "struct",
    "fields": [
      {
        "name": "indicate",
        "type": "u8"
      },
      {
        "name": "eventMarket",
        "type": {
          "option": "publicKey"
        }
      },
      {
        "name": "bump",
        "type": "u8"
      }
    ]
  }
}
```

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("marker"),
        user_pubkey.toBuffer(),
        sbt_mint_pubkey.toBuffer(),
    ],
    program.programId
);
```

#### mint

- 一般地，它往往指代FT Token（$MONI）的mint
- 可能有特别instruction中，它指代sbt_mint
- Token2022 的mint类型

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("mint"),
    ],
    program.programId
);
```

#### oracle_config

- 负责Oracle的配置信息
- 一一对应oracle_data和event_config
- 在demo场景中，只有恐惧贪婪指数这一个oracle_config

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("oracle-config"), 
        name.toBuffer(), // oracle name在demo中会是一个固定的值
    ],
    MONITOR_PROGRAM_ID // !!!!! monitor program id !!!!!
);
```

#### oracle_data

- 负责Oracle的具体数据
  一一对应oracle_config和event_config

**struct**

因为对于前端开发，这个结构可能有用，所以一并提供。

说明：

- rawData：恐惧贪婪指数的数据
- timestamp：这个数据的时间戳
- previousTimestamp：上一次提供数据的时间戳，用来反差上一次的数据


```json
    {
      "name": "oracleData",
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
            "name": "previousTimestamp",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "u64"
          },
          {
            "name": "slot",
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

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("oracle-data"),
        config_pubkey.toBuffer(), // oracle config 的pubkey
    ],
    MONITOR_PROGRAM_ID // !!!!! monitor program id !!!!!
);
```

#### payer

- 发起交易的客户端钱包，也负责支付创建所有的Account

**pda**

无，此Account是一个System Account

#### rent

- 一个特定的sysvar账户，用来计算Account的最小租金

**pda**

无，它是一个SysVar

```ts
anchor.web3.SYSVAR_RENT_PUBKEY
```

#### resolver

- 用来Finalize event market的账户，将调用oracle_data的值来更新event_market中的result

**pda**

无，此Account是一个System Account

#### sbt_mint

- 一般地，是指项目主SBT类别的mint账号，每个用户都会有自己的sbt_mint
- 属于token 2022的Mint类型，实现了non_transferable的Extension

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("sbt_mint"),
        user_pubkey.toBuffer(), // 用户的pubkey
    ],
    program.programId
);
```

#### sbt_mint_token_account

- 一般地，是指项目主SBT类别的用户个人token account
- 属于token 2022的TokenAccount类型

**pda**

```ts
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

const sbt_mint_token_account = await getAssociatedTokenAddress(
    sbt_mint_pubkey, user_pubkey, true,
    TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
```

#### system_program

- Solana系统 Program

**pda**

无，这是一个Program

```ts
anchor.web3.SystemProgram.programId
```

#### sysvar_instruction

- sysvar账户是一组特殊的内置账户，它们包含了全局状态和链上数据，供所有智能合约的实例访问。

**pda**

无，这是一个SysVar

```ts
anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
```

#### token_account

- 一般地，它是指FT Token（$MONI）对应user的TokenAccount
- 在一些instructions中，它可能也指代了sbt_mint的TokenAccount
- Token 2022的TokenAccount类型

**pda**

```ts
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

const token_account = await getAssociatedTokenAddress(
    mint_pubkey, user_pubkey, true,
    TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
```

#### token_metadata_program

- Metaplex Program

**pda**

无，这是一个Program

```ts
import {
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
```

#### token_program

- SPL Program
- 在本项目中使用的都是Token 2022 Program

**pda**

无，这是一个Program

```ts
    const MPL_TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
```


#### UserPosition(user_position)

- 在某个event_market下，用户的下注情况
- 如果某个event_market下，user_position不存在，则用户没有下注

**struct**

因为对于前端开发，这个结构可能有用，所以一并提供。

说明：

- existed：true说明存在，一般存在user_position就不会有false的情况，因为这样的account都会被close掉
- marker：下注后对应的marker pubkey

```json
    {
      "name": "userPosition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "positionType",
            "type": {
              "defined": "PositionType"
            }
          },
          {
            "name": "existed",
            "type": "bool"
          },
          {
            "name": "marker",
            "type": "publicKey"
          }
        ]
      }
    }
```

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from("user_position"),
        event_market_pubkey.toBuffer(),
        user_pubkey.toBuffer(),
    ],
    program.programId
);
```


#### vision_mining_admin

- 负责视觉挖矿的管理员

**pda**

无，此Account是一个System Account

#### vision_mining_pda

- 负责视觉挖矿的转账发起

**pda**

```ts
const [account_pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ft_vision_mining_pda"),
    ],
    program.programId // factory program id
);
```

## scratch_pad



1. 读取OracleData Account中的数据， raw_data
- 2. 获取景观的历史数据：slot（块高）/ timestamp来看， 一天会有一次数据更新。 getAccountInfoAtSlot
- 3. 获取市场：一个景观对应一个OracleConfig，对应一个OracleData，对应一个EventConfig。
     eventconfig中有个index（后期改成UTC-0点时间戳），这是目前实例化了多少个市场，从0开始。（2能用）
     eventmarket是有eventconfig + UTC-0点时间戳 PDA出来的。
     后天和大后天的预测场次判断逻辑需要明确。
- 4. 主SBT Mint： 200000 CP Units
     create_sbt(string + user_pubkey), mint_sbt
     Marker（用来记录SBT的使用）
- 5. 选择一个market，选择一个option，用sbt下注/撤注  产生pda：user_position和event_position。
- 6. 通过event_position查看某个market的下注情况
- 7. claim，三种结果（增加CP Unit）：ClaimWithNonPrize, ClaimWithPrize, ClaimWithPrizeAndSBT

metaplex NFT有几个Account：

spl 模型：
1. mint account
2. token account(user account 的ATA)

metaplex
1. metadata： 记录NFT的URL，symbol这些；
2. master_edition：（Prints）无副本/有限副本/无限副本
3. edition：副本

SBT
1. spl 2022 的mint account（82个bytes） + Extension（不确定的字节数）
   non-transferable 170个bytes
2. token account(user account 的ATA)

主SBT，每个SBT都是一个master_edition。因为metadata里的数据是不一样，至少美术方面的traits。
event SBT，一共两个master_edition（对应恐惧和贪婪），每个人拿的是print的edition。

FT：
Mint唯一