# Asset Hub Integration

Asset Hub is a system chain on the Paseo test network for minting and managing assets.

## 1. Asset Hub Channel Setup

The bi-directional HRMP channel setup with Asset Hub is permissionless. It requires an XCM call from the parachain to the Relay Chain.

**Sudo XCM Call (Educhain Reference):**

```hex
0x0f001f000301000314000400000000070010a5d4e81300000000070010a5d4e80006000300c16678419c183c0ae8030000140d01000001003145
```

!!! warning "Fund Your Sovereign Account"
    You must top-up the parachain's **sovereign account on the Paseo relay chain** with PAS tokens to pay for fees and deposits.
    
    [Use this utility](https://www.shawntabrizi.com/substrate-js-utilities/) (ParaID to Address) to find the address.

## 2. Foreign Asset Registry

You can register your parachain's native token as a Foreign Asset on Asset Hub.

### Step A: Register Asset

This requires an XCM call from the parachain to Asset Hub invoking `foreignAssets.create`. Since this cannot be executed directly, it's wrapped in a sudo XCM call from the parachain.

**Sudo XCM Call:**

```hex
0x0f001f0003010100a10f03140004000100000700e876481713000100000700e876481700060382acd84fc542e03500010100f146007369626cbc11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000140d0100000100f146
```

### Step B: Create Metadata

Next, set the metadata (symbol, decimals, etc.) on Asset Hub.

**Sudo XCM Call:**

```hex
0x3511010100314520456475636861696e0c4544550a
```

!!! warning "Fund Your Sibling Account"
    You must top-up the parachain's **sibling account on Asset Hub** with PAS tokens to pay for fees and deposits.
    
    [Use this utility](https://www.shawntabrizi.com/substrate-js-utilities/) (ParaID to Sibling Address) to find the address.



