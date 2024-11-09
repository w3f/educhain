# Asset Hub Integration

Asset Hub is a system chain of Paseo test network which allows for minting and managing fungible
and non-fungible assets. 

## Asset Hub Channel Setup

Through [this PR](https://github.com/paritytech/polkadot-sdk/pull/3721), the bi-directional HRMP channel 
setup with Asset Hub became permissionless and can be done through an XCM call from the parachain to Rococo 
relaychain. Here is the sudo XCM call used by the educhain for reference
`0x0f001f000301000314000400000000070010a5d4e81300000000070010a5d4e80006000300c16678419c183c0ae8030000140d01000001003145`.

**You need to top-up the parachain's sovereign account on Paseo relay chain with PAS tokens to pay for the fees and
deposits**

You can use `ParaID` to **child** address uitility [here](https://www.shawntabrizi.com/substrate-js-utilities/) to determine which address 
on Paseo chain to send tokens to.

## Foreign Asset Registry

The native token of a parachain can be registered as a Foriegn Asset on Asset Hub. This can be accomplished 
through an XCM call from the parachain to the Asset Hub that invokes `create` call of the `foreignAssets` 
pallet. Here is the call that needed to be executed on the Asset Hub to register its native token as a foreign asset `0x3500010100f146007369626cbc11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000`. As this call cannot be directly executed on Paseo Asset Hub, it is wrapped in a sudo XCM call from the parachain 
`0x0f001f0003010100a10f03140004000100000700e876481713000100000700e876481700060382acd84fc542e03500010100f146007369626cbc11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000140d0100000100f146`

The next step is to create metadata for the asset on Asset Hub. Here is the call that needed to be executed 
on the Asset Hub via the XCM message from parachain `0x3511010100314520456475636861696e0c4544550a`. Repeat the same procedure as above.

**You need to top-up the parachain's sibling account on Paseo relay chain with PAS tokens to pay for the fees and
deposits**

You can use `ParaID` to **sibling** address uitility [here](https://www.shawntabrizi.com/substrate-js-utilities/) to determine which 
address on Paseo chain to send tokens to.



