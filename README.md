> 🚀 **Get Started**: Follow our comprehensive guide to set up your Gridlock guardian node and start earning rewards today! [Learn how to run a guardian node →](./running_a_guardian_node.md)

# Gridlock Guardian Node

The guardian node is the heart of the Gridlock network. It stores part of a user's private key and works together with other guardian nodes to approve transactions and recover accounts. This distributed approach removes single points of failure, which is the root cause of most crypto loss.

Anyone can run one or more guardian nodes. You can run them to protect your own assets or participate in Gridlock's main network and help protect others—earning rewards in the process.

To understand how the full system works, see [SystemOverview.md](./SystemOverview.md).  
Related: [Orch Node](https://github.com/GridlockNetwork/orch-node) | [SDK](https://github.com/GridlockNetwork/gridlock-sdk) | [CLI](https://github.com/GridlockNetwork/gridlock-cli)

## Quick Start

### Option 1: Full Setup (Recommended)

Use the [Gridlock CLI](https://github.com/GridlockNetwork/gridlock-cli) to run everything with a single command - orchestration node, guardian nodes, network, and database.

### Option 2: Guardian Node Only

If you already have an orchestration node running, you can run a single guardian node:

```
docker run --rm --name guardian-node --network gridlock-net \
  gridlocknetwork/guardian-node:latest
```

For advanced setups including multiple guardian nodes, see [Customization and Development Guide](./customization_and_development.md).

## Join the Network

Running a guardian node gets better when you're part of the official Gridlock network. Run a guardian node and earn rewards while helping others

FOLLOW THIS GUIDE to get started earning LOCK.

Get started learning about the community and LOCK token at [gridlock.network/join](https://gridlock.network/join)
