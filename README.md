# Gridlock Guardian Node

The guardian node is the heart of the Gridlock network. It stores part of a user's private key and works together with other guardian nodes to approve transactions and recover accounts. This distributed approach removes single points of failure, which is the root cause of most crypto loss.

Anyone can run one or more guardian nodes. You can run them to protect your own assets or participate in Gridlock's main network and help protect others—earning rewards in the process.

To understand how the full system works, see [SystemOverview.md](./SystemOverview.md).  
Related: [Orch Node](https://github.com/GridlockNetwork/orch-node) | [SDK](https://github.com/GridlockNetwork/gridlock-sdk) | [CLI](https://github.com/GridlockNetwork/gridlock-cli)

## Prerequisites

> ⚠️ **Important**: The guardian node requires the [Orchestration Node](https://github.com/GridlockNetwork/orch-node) and its associated services to be running. Make sure you have this set up before proceeding.

## Quick Start

### Configuration

Guardian nodes require a `.env` file for configuration. Copy the example configuration file:

```bash
cp example.env .env
```

Then edit the `.env` file to customize settings like storage directories and network configuration.

### Running a Single Guardian Node

If you already have an orchestration node running, you can run a single guardian node:

```bash
docker run --rm --name guardian-node --network gridlock-net \
  -v $(pwd)/.env:/app/.env:ro \
  gridlocknetwork/guardian-node:latest
```

### Running Multiple Guardian Nodes

For advanced setups including multiple guardian nodes, see [Customization and Development Guide](./customization_and_development.md).

> 🚀 **Pro Tip**: Want to run everything with a single command? Check out the [Gridlock CLI](https://github.com/GridlockNetwork/gridlock-cli) which can set up and run the entire Gridlock network stack - orchestration node, guardian nodes, network, and database - with just one command!

## Join the Network

Running a guardian node gets better when you're part of the official Gridlock network. Run a guardian node and earn rewards while helping others.

Get started learning about the community and LOCK token at [gridlock.network/join](https://gridlock.network/join)
