# Integrating with Asset Hub

Asset Hub is a system chain of Rococo test network which allows for minting and managing fungible
and non-fungible assets. 

## Establish Channels with Asset Hub

Through [this PR](https://github.com/paritytech/polkadot-sdk/pull/3721), the bi-directional HRMP channel 
setup with Asset Hub became permissionless and can be done through an XCM call from the parachain to Rococo 
relaychain. Here is the sudo XCM call used by the educhain for reference
`0x0f001f000301000314000400000000070010a5d4e81300000000070010a5d4e80006000300c16678419c183c0ae8030000140d01000001003145`.

## Register as Foreign Asset

The native token of a parachain can be registered as a Foriegn Asset on Asset Hub. This can be accomplished 
through an XCM call from the parachain to the Asset Hub that invokes `create` call of the `foreignAssets` 
pallet. Here is the call that needed to be executed on the Asset Hub to register its native token as a foreign asset `0x3500010100314500706172614c11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000`. As this call cannot be directly executed on Rococo Asset Hub, it is wrapped in a sudo XCM call from the educhain 
`0x0f001f0003010100a10f0314000400010000070010a5d4e81300010000070010a5d4e80006030248fa7b419ce03500010100314500706172614c11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000140d01000001003145`

The next step is to create metadata for the asset on Asset Hub. Here is the call that needed to be executed 
on the AssetHub via the XCM message from Educhain `0x3511010100314520456475636861696e0c4544550a` and here is 
the sudo XCM call used by the educhain to embed that call
`0x0f001f0003010100a10f0314000400010000070010a5d4e81300010000070010a5d4e800060302389c1c419ce03500010100314500706172614c11000000000000000000000000000000000000000000000000000000ca9a3b000000000000000000000000140d01000001003145`



