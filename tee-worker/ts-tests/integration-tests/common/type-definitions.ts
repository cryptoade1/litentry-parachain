import { ApiPromise } from '@polkadot/api';
import { KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';
import { Metadata, TypeRegistry } from '@polkadot/types';
import { Wallet } from 'ethers';
import type { PalletIdentityManagementTeeIdentityContext, LitentryPrimitivesIdentity } from 'sidechain-api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { HexString } from '@polkadot/util/types';

export type EnclaveResult = {
    mrEnclave: HexString;
    shieldingKey: HexString;
    vcPubkey: HexString;
    sgxMetadata: object;
};
export type PubicKeyJson = {
    n: Uint8Array;
    e: Uint8Array;
};

interface EthersWalletItem {
    [key: string]: Wallet;
}
interface SubstrateWalletItem {
    [key: string]: KeyringPair;
}
export type IntegrationTestContext = {
    tee: WebSocketAsPromised;
    api: ApiPromise;
    teeShieldingKey: KeyObject;
    mrEnclave: HexString;
    ethersWallet: EthersWalletItem;
    substrateWallet: SubstrateWalletItem;
    sidechainMetaData: Metadata;
    sidechainRegistry: TypeRegistry;
    web3Signers: Web3Wallets[];
    chainIdentifier: number;
};

export class AesOutput {
    ciphertext?: Uint8Array;
    aad?: Uint8Array;
    nonce?: Uint8Array;
}

export type Web3Wallets = {
    substrateWallet: KeyringPair;
    ethereumWallet: Wallet;
};

export type IdentityGenericEvent = {
    who: HexString;
    identity: LitentryPrimitivesIdentity;
    idGraph: [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][];
};

export enum IdentityStatus {
    Active = 'Active',
    Inactive = 'Inactive',
}

export type IdentityContext = {
    link_block: number;
    status: IdentityStatus;
};

//vc types
export type VcRequested = {
    account: HexString;
    mrEnclave: HexString;
    assertion: Assertion;
};

export enum RequestEvent {
    LinkIdentityRequested = 'LinkIdentityRequested',
    SetUserShieldingKeyRequested = 'SetUserShieldingKeyRequested',
    DeactivateIdentityRequested = 'DeactivateIdentityRequested',
    ActivateIdentityRequested = 'ActivateIdentityRequested',
    VCRequested = 'VCRequested',
    ItemCompleted = 'ItemCompleted',
    BatchCompleted = 'BatchCompleted',
}

export type Assertion =
    | { A1: string }
    | { A2: [string] }
    | { A3: [string, string, string] }
    | { A4: string }
    | { A5: [string, string] }
    | { A6: string }
    | { A7: string }
    | { A8: [string] }
    | { A9: string }
    | { A10: string }
    | { A11: string };

export type TransactionSubmit = {
    tx: SubmittableExtrinsic<ApiTypes>;
    nonce: number;
};

//call types
export type RequestBody = {
    id: number;
    jsonrpc: string;
    method: string;
};

export const jsonSchema = {
    type: 'object',
    properties: {
        id: {
            type: 'string',
        },
        type: {
            type: 'array',
        },
        issuer: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                name: {
                    type: 'string',
                },
                shard: {
                    type: 'string',
                },
            },
        },
        issuanceDate: {
            type: 'string',
        },
        credentialSubject: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                description: {
                    type: 'string',
                },
                type: {
                    type: 'string',
                },
                tag: {
                    type: 'array',
                },
                assertions: {
                    type: 'array',
                    items: {
                        type: 'object',
                    },
                },
                values: {
                    type: 'array',
                    items: {
                        type: 'boolean',
                    },
                },
                endpoint: {
                    type: 'string',
                },
            },
            required: ['id', 'description', 'type', 'assertions', 'values', 'endpoint'],
        },
        proof: {
            type: 'object',
            properties: {
                created: {
                    type: 'string',
                },
                type: {
                    enum: ['Ed25519Signature2020'],
                },
                proofPurpose: {
                    enum: ['assertionMethod'],
                },
                proofValue: {
                    type: 'string',
                },
                verificationMethod: {
                    type: 'string',
                },
            },
        },
    },
    required: ['id', 'type', 'credentialSubject', 'issuer', 'issuanceDate', 'proof'],
};
