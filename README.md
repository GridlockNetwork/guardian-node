# Gridlock Guardian Node

The guardian node is the heart of the Gridlock network. It stores part of a user's private key and works together with other guardian nodes to approve transactions and recover accounts. This distributed approach removes single points of failure, which is the root cause of most crypto loss.

Anyone can run one or more guardian nodes. You can run them to protect your own assets or participate in Gridlock's main network and help protect others—earning rewards in the process.

To understand how the full system works, see [SystemOverview.md](./SystemOverview.md).  
Related: [Orch Node](https://github.com/GridlockNetwork/orch-node) | [SDK](https://github.com/GridlockNetwork/gridlock-sdk) | [CLI](https://github.com/GridlockNetwork/gridlock-cli)

## Quick Start

Run the official Docker image:

```
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

By default, the node uses the configuration built into the image from `example.env`. To customize it, use your own `.env` file.

We recommend storing your config at `/Users/USERNAME/.gridlock-guardian-node/.env` (replace `USERNAME` accordingly).

### Using Command Line

```
# Run with custom config
docker run --rm -v /Users/USERNAME/.gridlock-guardian-node/.env:/app/.env gridlocknetwork/guardian-node:latest
```

### Using Docker Compose

You will need a `.env` file and a nats config file:

```sh
cat << EOF >.env
NODE_DB=/var/lib/gridlock/node/node.db
NATS_ADDRESS=nats://nats:4222
NATS_ROLE=ruser
NATS_PASS=T0pS3cr3t
EOF

mkdir -p nats
cat << EOF >nats/nats.cfg
# Client port of 4222 on all interfaces
port: 4222

# HTTP monitoring port
monitor_port: 8222

# This is for clustering multiple servers together.
cluster {
  # It is recommended to set a cluster name
  name: "my_cluster"

  # Route connections to be received on any interface on port 6222
  port: 6222

  # Routes are protected, so need to use them with --routes flag
  # e.g. --routes=nats-route://ruser:T0pS3cr3t@otherdockerhost:6222
  authorization {
    user: ruser
    password: T0pS3cr3t
    timeout: 2
  }

  # Routes are actively solicited and connected to from this server.
  # This Docker image has none by default, but you can pass a
  # flag to the nats-server docker image to create one to an existing server.
  routes = []
}
EOF
```

Then just: `docker compose up -d`

## How It Works

The guardian node communicates with the [orch-node](https://github.com/GridlockNetwork/orch-node) to coordinate signing and recovery. It holds one key share and uses threshold signature cryptography, meaning several guardians must work together to authorize actions. This ensures that no single node can compromise a user's wallet—even if it's lost or attacked.

The node uses internal storage inside the container and does not require additional persistent volumes.

For usage with other tools, check out:

- [gridlock-sdk](https://github.com/GridlockNetwork/gridlock-sdk)
- [gridlock-cli](https://github.com/GridlockNetwork/gridlock-cli)

## Join the Network

This code is yours to use — but it’s even better when you’re part of the official Gridlock network.

By running guardian nodes, you can earn rewards while helping secure the network.

Join the community: [gridlock.network/join](https://gridlock.network/join)
