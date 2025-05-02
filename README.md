> 🚀 **Get Started**: Follow our comprehensive guide to set up your Gridlock guardian node and start earning rewards today! [Learn how to run a guardian node →](./running_a_guardian_node.md)

# Gridlock Guardian Node

The guardian node is the heart of the Gridlock network. It stores part of a user's private key and works together with other guardian nodes to approve transactions and recover accounts. This distributed approach removes single points of failure, which is the root cause of most crypto loss.

Anyone can run one or more guardian nodes. You can run them to protect your own assets or participate in Gridlock's main network and help protect others—earning rewards in the process.

To understand how the full system works, see [SystemOverview.md](./SystemOverview.md).  
Related: [Orch Node](https://github.com/GridlockNetwork/orch-node) | [SDK](https://github.com/GridlockNetwork/gridlock-sdk) | [CLI](https://github.com/GridlockNetwork/gridlock-cli)

## Quick Start

The easiest way to see the full setup in action is to use the [Gridlock CLI](https://github.com/GridlockNetwork/gridlock-cli), which can run both the orchestration node and three guardian nodes with a single command.

Alternatively, you can run the single Docker image:

```
docker run --rm --name guardian-node --network gridlock-net \
  gridlocknetwork/guardian-node:latest
```

To run three guardian nodes (useful when running the example in [gridlock-cli](https://github.com/GridlockNetwork/gridlock-cli)), you can use docker compose:

```
docker compose up
```

For detailed customization options, local development setup, and advanced configuration, see [Customization and Development Guide](./customization_and_development.md).

## Join the Network

Running a guardian node gets better when you're part of the official Gridlock network. Run a guardian node and earn rewards while helping others

FOLLOW THIS GUIDE to get started earning LOCK.

Get started learning about the community and LOCK token at [gridlock.network/join](https://gridlock.network/join)
