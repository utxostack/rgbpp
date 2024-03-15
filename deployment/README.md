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

Step 2, edit `build/deployment-configs/btc-time-lock.toml`. Set lock of output cells.

Step 3, deploy `btc-time-lock`.

Step 4, generate RGBPP config binaries.

```bash 
cd rgbpp

# use deploy-tool to gen config binary

# RGBPP config
cargo run --bin deploy-tool -- -o deploy/config-bins/rgbpp-config gen-rgbpp-config --btc-lc-type-hash <32 bytes type hash> --btc-time-lock-type-hash <32 bytes type hash>
```

Step 5, edit `build/deployment-configs/rgbpp-lock.toml`. Set lock of output cells.

Step 6, deploy `rgbpp-lock`.

