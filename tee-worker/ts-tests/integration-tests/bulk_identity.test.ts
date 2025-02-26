import { step } from 'mocha-steps';
import {
    buildValidations,
    describeLitentry,
    buildIdentityTxs,
    buildIdentityHelper,
    buildIdentityFromKeypair,
    batchAndWait,
    PolkadotSigner,
} from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
import type { LitentryPrimitivesIdentity } from 'sidechain-api';
import { assert } from 'chai';
import { multiAccountTxSender } from './common/transactions';
import type { Web3Network } from 'parachain-api';

//Explain how to use this test, which has two important parameters:
//1.The "number" parameter in describeLitentry represents the number of accounts generated, including Substrate wallets and Ethereum wallets.If you want to use a large number of accounts for testing, you can modify this parameter.
//2.Each time the test code is executed, new wallet account will be used.

describeLitentry('multiple accounts test', 2, async (context) => {
    let substrateSigners: KeyringPair[] = [];
    let ethereumSigners: ethers.Wallet[] = [];
    const identities: LitentryPrimitivesIdentity[] = [];
    step('setup signers', async () => {
        substrateSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.substrateWallet;
        });
        ethereumSigners = context.web3Signers.map((web3Signer) => {
            return web3Signer.ethereumWallet;
        });
    });

    step('send test token to each account', async () => {
        await batchAndWait(
            context.api,
            context.substrateWallet.alice,
            substrateSigners.map((signer) => context.api.tx.balances.transfer(signer.address, '1000000000000'))
        );
    });

    //test with multiple accounts
    step('test set usershieldingkey with multiple accounts', async () => {
        const txs = await buildIdentityTxs(context, substrateSigners, [], 'setUserShieldingKey');
        await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', ['UserShieldingKeySet']);
    });

    //test identity with multiple accounts
    step('test linkIdentity with multiple accounts', async () => {
        const web3networks: Web3Network[][] = [];
        const signerIdentities: LitentryPrimitivesIdentity[] = [];
        const defaultNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum']) as unknown as Web3Network[];

        for (let index = 0; index < ethereumSigners.length; index++) {
            const signerIdentity = await buildIdentityFromKeypair(new PolkadotSigner(substrateSigners[index]), context);
            const identity = await buildIdentityHelper(ethereumSigners[index].address, 'Evm', context);
            identities.push(identity);
            web3networks.push(defaultNetworks);
            signerIdentities.push(signerIdentity);
        }

        const validations = await buildValidations(
            context,
            signerIdentities,
            identities,
            2,
            'ethereum',
            undefined,
            ethereumSigners
        );

        const txs = await buildIdentityTxs(
            context,
            substrateSigners,
            identities,
            'linkIdentity',
            validations,
            web3networks
        );

        const events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'IdentityLinked',
        ]);
        assert.equal(events.length, txs.length, 'verify identities with multiple accounts check fail');
    });

    step('test deactivateIdentity with multiple accounts', async () => {
        const txs = await buildIdentityTxs(context, substrateSigners, identities, 'deactivateIdentity');

        const events = await multiAccountTxSender(context, txs, substrateSigners, 'identityManagement', [
            'IdentityDeactivated',
        ]);
        assert.equal(events.length, txs.length, 'deactivate identities with multiple accounts check fail');
    });
});
