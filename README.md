# Gridlock Guardian Node

The guardian node is the heart of the Gridlock network. It stores part of a user's private key and works together with other guardian nodes to approve transactions and recover accounts. This distributed approach removes single points of failure, which is the root cause of most crypto loss.

Anyone can run one or more guardian nodes. You can run them to protect your own assets or participate in Gridlock's main network and help protect others—earning rewards in the process.

To understand how the full system works, see [SystemOverview.md](./SystemOverview.md).  
Related: [Orch Node](https://github.com/GridlockNetwork/orch-node) | [SDK](https://github.com/GridlockNetwork/gridlock-sdk) | [CLI](https://github.com/GridlockNetwork/gridlock-cli)

## Quick Start

Run the official Docker image:

```
docker pull gridlocknetwork/guardian-node:latest
docker run --rm --name guardian-node --network gridlock-net \
  gridlocknetwork/guardian-node:latest
```

To run three guardian nodes (useful when running the example in [gridlock-cli](https://github.com/GridlockNetwork/gridlock-cli)):

```
for i in 1 2 3; do
  docker run --rm --name guardian-node-$i --network gridlock-net \
    gridlocknetwork/guardian-node:latest &
done
```

To provide your own configuration, mount a `.env` file as shown below.

## Configuration

By default, the node uses the configuration built into the image from `.env.example`. To customize it, use your own `.env` file.

We recommend storing your config at `/Users/USERNAME/.gridlock-guardian-node/.env` (replace `USERNAME` accordingly).

### Using Docker Desktop

1. Open Docker Desktop
2. Pull the `gridlocknetwork/guardian-node` image
3. Go to the "Images" tab
4. Find `gridlocknetwork/guardian-node:latest`
5. Click "Run"
6. Under "Volumes", click "Add volume":
   - **Host path**: `/Users/USERNAME/.gridlock-guardian-node/.env`
   - **Container path**: `/app/.env`
7. Click "Run"

### Using Command Line

```
# Run with user config (recommended)
docker run --rm -v /Users/USERNAME/.gridlock-guardian-node/.env:/app/.env gridlocknetwork/guardian-node:latest

# Run with default config
docker run --rm gridlocknetwork/guardian-node:latest
```

---

## Development

If you want to build or modify the guardian node, clone the repo and follow the instructions in the development section.

---

## How It Works

The guardian node communicates with the [orch-node](https://github.com/GridlockNetwork/orch-node) to coordinate signing and recovery. It holds one key share and uses threshold signature cryptography, meaning several guardians must work together to authorize actions. This ensures that no single node can compromise a user's wallet—even if it's lost or attacked.

The node uses internal storage inside the container and does not require additional persistent volumes.

For usage with other tools, check out:

- [gridlock-sdk](https://github.com/GridlockNetwork/gridlock-sdk)
- [gridlock-cli](https://github.com/GridlockNetwork/gridlock-cli)

---

## Join the Network

Want to run a node on the public Gridlock network?  
Join the community: [gridlock.network/join](https://gridlock.network/join)
