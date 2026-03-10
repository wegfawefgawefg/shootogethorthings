# Networking Notes

`shootogethorthings` is a small November 2023 experiment around multiplayer game networking, not a finished netcode stack.

## What It Was Trying To Do

- Cooperative play, not competitive play.
- Stay playable under very high latency.
- Bias toward local responsiveness instead of strict server authority.
- Use a small Rust client/server setup with `tokio`, `UDP`, and ECS-shaped game state.

## What Is Sound About The Direction

- `UDP` is a reasonable transport choice for realtime action games.
- A fully authoritative "server decides every frame" model would feel bad under very high latency.
- For long-distance co-op, a hybrid or local-first approach makes sense:
  - local input and movement feel immediate
  - important shared events still need arbitration
  - some mismatch is acceptable if the game is cooperative

## What Is Fundamentally Weak In This Repo

- The actual sync model is incomplete.
- Clients lean toward sending state instead of sending inputs.
- The server is not cleanly authoritative over shared world truth.
- Client reconciliation / correction is mostly missing.
- Snapshot flow is incomplete.
- Several async loops are hot busy-loops, which wastes CPU.

So while the transport direction is fine, the repo stops at "networked prototype" instead of becoming a robust hybrid multiplayer architecture.

## A Better Direction For This Kind Of Game

For high-latency cooperative action, the saner model is:

1. Clients send inputs or high-level intents.
2. Local player movement and shooting happen immediately on each client.
3. A host or server arbitrates shared world truth where it matters:
   - enemy health
   - pickups consumed
   - mission progress
   - spawned entities
   - major events
4. Server or host sends periodic snapshots.
5. Clients reconcile gently and interpolate remote entities.

That is still "authoritative" in a limited sense, but it avoids the dead feel of waiting for remote confirmation for every action.

## Bottom Line

This repo is useful as a small networking fossil:

- right transport choice for the genre target
- reasonable instinct toward local-first co-op
- incomplete and buggy execution

It should be read as an experiment, not as a reference implementation.
