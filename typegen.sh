#!/bin/bash
rootDir=$(pwd)

echo "Root Path: $rootDir"

contractPaths=(
  'contracts/proposal/proposal-nft-voting-factory'
  'contracts/proposal/proposal-nft-voting'
)
contractNames=(
'ProposalFactory'
'Proposal'
)

num=${#contractPaths[@]}
for ((i=0; i<$num; i++))
  do
    path=${contractPaths[i]}
    name=${contractNames[i]}
    cd "$path" || exit 1
    # Update schema
    cargo schema
    # Run TS codegen
    cosmwasm-ts-codegen generate \
      --schema ./schema \
      --out ./ts \
      --name "$name"
    cd "$rootDir" || exit 1
  done
