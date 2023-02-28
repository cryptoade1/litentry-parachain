#!/bin/bash

# Copyright 2020-2023 Litentry Technologies GmbH.

while getopts ":p:A:B:u:W:V:C:" opt; do
    case $opt in
        p)
            NPORT=$OPTARG
            ;;
        A)
            WORKER1PORT=$OPTARG
            ;;
        B)
            WORKER2PORT=$OPTARG
            ;;
        u)
            NODEURL=$OPTARG
            ;;
        V)
            WORKER1URL=$OPTARG
            ;;
        W)
            WORKER2URL=$OPTARG
            ;;
        C)
            CLIENT_BIN=$OPTARG
            ;;
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/integritee-cli"}

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

ACC=//Bob
KEY="22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12"

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is $CLIENT"

echo "* Query on-chain enclave registry:"
WORKERS=$($CLIENT list-workers)
echo "WORKERS: "
echo "${WORKERS}"
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # This will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $(echo "$WORKERS" | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }

sleep 5
echo "* Set $ACC 's shielding key to $KEY"
${CLIENT} trusted --mrenclave $MRENCLAVE --direct set-user-shielding-key-preflight "$ACC" "$KEY"

sleep 20

echo "* Build assertion for $ACC"
${CLIENT} trusted --mrenclave $MRENCLAVE --direct build-assertion "$ACC" a1
echo ""

