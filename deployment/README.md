# Deployment

## ckb-cli

Install ckb-cli, ensure version is `1.7.0`

``` bash
ckb-cli --version
# ckb-cli 1.7.0 (149cd50 2024-03-05)
```

## Prepare

```bash
# root dir
cd rgbpp
# Create a deploy directory
mkdir deploy

# build
make build

# Copy binaries
cp -r build deploy/build
# Copy deployment config
cp -r deployment/deployment-configs deploy/deployment-configs
# create dir
mkdir deploy/config-bins
```

## Config

Step 1, generate config binaries.

```bash 
cd rgbpp

# use deploy-tool to gen config binary

# btc time lock config
cargo run --bin deploy-tool -- -o deploy/config-bins/btc-time-lock-config gen-btc-time-lock-config --btc-lc-type-hash <32 bytes type hash>
```

Step 2, edit `deploy/deployment-configs/btc-time-lock.toml`. Set lock of output cells.

Step 3, deploy `btc-time-lock`, see ckb-cli deploy command.

Step 4, generate RGBPP config binaries.

```bash 
cd rgbpp

# use deploy-tool to gen config binary

# RGBPP config
cargo run --bin deploy-tool -- -o deploy/config-bins/rgbpp-config gen-rgbpp-config --btc-lc-type-hash <32 bytes type hash> --btc-time-lock-type-hash <32 bytes type hash>
```

Step 5, edit `deploy/deployment-configs/rgbpp-lock.toml`. Set lock of output cells.

Step 6, deploy `rgbpp-lock`, see ckb-cli deploy command.


## ckb-cli deploy command

``` bash
# An example of deploy command

# make a directory

mkdir rgbpp-deploy && cd rgbpp-deploy

# or mkdir btc-time-lock-deploy && cd btc-time-lock-deploy

mkdir migrations

# Generate deployment transaction
ckb-cli --url https://testnet.ckbapp.dev/ deploy gen-txs --deployment-config ../deployment-configs/btc-time-lock.toml --fee-rate 1000 --from-address ckt1<address to pay transaction fee> --info-file deploy_info.json --migration-dir migrations

# Sign transaction
ckb-cli --url https://testnet.ckbapp.dev/ deploy sign-txs --add-signatures --info-file deploy_info.json --from-account <20 bytes lock args>

# Send transaction
ckb-cli --url https://testnet.ckbapp.dev/ deploy apply-txs --info-file deploy_info.json --migration-dir migrations/

# Wait few blocks to confirm transaction is submitted
```

Cell's `type_id` is located in `deploy_info.json` file, under the key `new_recipes[0].type_id`, you can also check cell recipes's name.

