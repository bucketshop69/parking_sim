# 🅿️ Parking Sim Architecture & Design Document

> **Status:** Architecture Complete  
> **Last Updated:** December 25, 2025  
> **Authors:** Co-founders brainstorm session

---

## 🎯 The Vision

**Clash of Clans meets Solana** — A parking lot management game where the backend IS the blockchain.

---

## 📋 Topics Summary

| # | Topic | Status |
|---|-------|--------|
| 1 | Game Loop & Core Mechanics | ✅ Locked |
| 2 | Player Progression | ✅ Locked |
| 3 | Bot System | ✅ Locked |
| 4 | Economy & Tokenomics | ✅ Locked |
| 5 | On-Chain vs Off-Chain | ✅ Locked |
| 6 | Ephemeral Rollups | ⏸️ Deferred (post-MVP) |
| 7-9 | Multiplayer, Monetization, Tech Stack | ⏸️ Out of Scope |
| 10 | MVP Scope | ✅ Locked |

---

# 1. Game Loop & Core Mechanics

## Decisions

| Decision | Choice |
|----------|--------|
| Active vs Idle | **Active** — Player must be online |
| Overflow handling | **Patience Timer** — Bots wait, then leave |
| Skill expression | **Prioritization + Matching** |
| Depth model | **Time Pressure + Matching + Facility Limits** |

## Depth Model

```
┌─────────────────────────────────────────────────────────────────────────┐
│   LAYER 1: TIME PRESSURE                                                │
│   • Different bot types have different PATIENCE and PAYOUT              │
│   • Bots leave if not assigned in time (missed revenue)                 │
│   • Player must prioritize: VIP (fast, high pay) vs Basic (slow, safe)  │
│                                                                         │
│   LAYER 2: MATCHING                                                     │
│   • Bots want different FACILITIES (garage, shop, etc.)                 │
│   • Correct facility match = bonus payout                               │
│   • Facilities unlock at higher levels                                  │
│                                                                         │
│   LAYER 3: FACILITY CAPACITY                                            │
│   • Facilities have limited throughput                                  │
│   • Must balance: quick turnover vs. high-value long stays              │
│                                                                         │
│   RESULT: Casual (random = works) vs Skilled (optimize = 2-3x more)    │
└─────────────────────────────────────────────────────────────────────────┘
```

## First-Time User Flow (Tutorial)

```
1. Player connects wallet (no game state detected)
2. Tutorial triggered automatically
3. Player sees: 2 starter spots (guided, can't skip)
4. Bot spawns → "Let this bot park in your lot" prompt
5. Player assigns bot to spot (learns core action)
6. Quick 30-second wait
7. Bot leaves → Player receives first $PARK tokens! 🎉
8. Progression pitch shown
9. Tutorial complete → Free play begins
```

**Key Insight:** Tutorial bots pay with "system-minted" $PARK — the only legitimate faucet, gated behind onboarding.

## Core Loop (Post-Tutorial)

```
BOTS ARRIVE → PLAYER ASSIGNS → BOTS PARK → BOTS LEAVE (pay $PARK) → PLAYER UPGRADES → Loop
```

---

# 2. Player Progression

## Decisions

| Decision | Choice |
|----------|--------|
| Progression type | **Infinite levels** (add more over time like CoC) |
| Level design | **Hand-crafted per level** (no formula) |
| Upgrade mechanic | **Municipality Approval** (Request → Wait → Approved) |
| Wait time scaling | **Variable by level** (small = fast, big = slow) |
| Facilities | **Separate unlocks with rent** |
| Rent missed | **Soft punishment** (facility closed, base pay only) |

## Design Philosophy

**Progression should feel EARNED, not BOUGHT.**

- Skill and time matter more than wallet size
- Each level feels intentionally designed
- You can't just pay-to-skip the experience

## Municipality Approval System

```
1. Player reaches requirements (enough $PARK)
2. Player submits "Request for Upgrade" to Municipality
3. Wait time begins (varies by upgrade size)
4. Approval granted → Player pays $PARK → Upgrade applied
```

## Facility Rental System

```
UNLOCK → Pay one-time unlock fee → Facility available
ACTIVE → Pay monthly rent in $PARK → Earn premium rates
RENT MISSED → Facility "CLOSED" → Bots can't use it → Base pay only
```

---

# 3. Bot System

## Decisions

| Decision | Choice |
|----------|--------|
| Bot spawn gating | **Level + Facility (AND)** |
| Spot types | **Regular + VIP** |
| VIP spot progression | **Level milestone** |
| Bonus system | **Adaptive** (for events/tuning) |
| Special/Event bots | **Post-MVP** |

## Bot Lifecycle

```
PHASE 1: ARRIVAL    → Bot spawns, enters queue, patience timer starts
PHASE 2: WAITING    → Timer ticking, leaves ANGRY if hits 0
PHASE 3: ASSIGNED   → Player assigns to spot, stay timer starts
PHASE 4: PARKED     → Bot occupies spot, timer counting down
PHASE 5: CHECKOUT   → Bot pays $PARK, spot is FREE again
```

## Bot Spawn Logic

| Bot Type | Level Required | Facility Required |
|----------|----------------|-------------------|
| Basic | 1+ | None |
| Shopper | 5+ | Shop unlocked |
| Repair | 8+ | Garage unlocked |
| VIP | 3+ | None (needs VIP spot) |

## Spot Types

| Spot Type | Multiplier | Unlock |
|-----------|------------|--------|
| Regular | 1x | Default |
| VIP | Higher (adaptive) | Level milestone |

## Payout Formula

```
PAYOUT = Base_Rate × Bot_Multiplier × Spot_Multiplier × Facility_Bonus
```

| Component | Values |
|-----------|--------|
| Bot Multiplier | Basic: 1x, Shopper: 1.5x, Repair: 2x, VIP: 5x |
| Spot Multiplier | Regular: 1x, VIP: 1.5x (adaptive) |
| Facility Bonus | No facility: 1x, Uses facility: +0.3x |

## Bot Types Summary

| Bot Type | Patience | Payout | Facility | Stay Duration | Risk/Reward |
|----------|----------|--------|----------|---------------|-------------|
| Basic | 60s | 1x | None | Short | Low/Low |
| Shopper | 45s | 1.5x | Shop | Medium | Med/Med |
| Repair | 30s | 2x | Garage | Long | Med/High |
| VIP | 10s | 5x | VIP Spot | Short | High/High |

---

# 4. Economy & Tokenomics

## Decisions

| Decision | Choice |
|----------|--------|
| Token supply | **Fixed cap (100M $PARK)** |
| Mint authority | **Program-controlled** |
| Bot payments | **From treasury** |
| Sink destination | **Back to treasury** (circular) |
| SOL bonds | **Lock-only (no risk)** |
| DEX management | **Not our problem** |
| Dual currency | **$PARK (soft) + SOL (hard)** |

## Token Economy

```
SUPPLY: 100,000,000 $PARK (fixed, enforced by program)

DISTRIBUTION:
• Treasury: 80% (80M) — for bot payouts
• Team: 10% (10M) — for development
• Reserve: 10% (10M) — for future features

FAUCETS (into player wallets):
• Bot checkout payments (from treasury)
• Tutorial completion (one-time)

SINKS (out of player wallets):
• Level upgrade costs → back to treasury
• Facility unlock fees → back to treasury
• Monthly rent payments → back to treasury

CIRCULAR FLOW:
Treasury → Bots pay players → Players spend on upgrades → Treasury
```

## SOL Bond System

```
OPEN FACILITY  → Lock X SOL as bond → Facility active
FACILITY ACTIVE → SOL remains locked → Earn premium $PARK
CLOSE FACILITY → SOL bond returned in full
```

## DEX Philosophy

We control: Total supply, mint authority, how players earn/spend $PARK
We don't control: DEX listings, trading price, speculation

**"We focus on the GAME, not the SPECULATION."**

---

# 5. On-Chain vs Off-Chain

## Decisions

| Decision | Choice |
|----------|--------|
| MVP approach | **Everything on Mainnet** |
| Ephemeral rollups | **Future (post-MVP)** |
| VRF | **On-chain (mainnet)** |

## MVP Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     SOLANA MAINNET                          │
│   • $PARK token + treasury                                  │
│   • SOL bonds (locked)                                      │
│   • Lot state (level, spots, facilities)                    │
│   • Bot spawns (VRF), assignments, checkout                 │
│   • Upgrades and facility unlocks                           │
└─────────────────────────────────────────────────────────────┘
                            ↑↓
┌─────────────────────────────────────────────────────────────┐
│                     CLIENT (Off-chain)                      │
│   • Graphics / animations / sound                           │
│   • UI state (menus, buttons)                               │
│   • Local caching                                           │
└─────────────────────────────────────────────────────────────┘
```

## MVP Tradeoffs (Acknowledged)

| Tradeoff | Impact | Mitigation |
|----------|--------|------------|
| Slower transactions | ~400ms per action | Acceptable for MVP |
| Transaction costs | SOL fees per action | Keep actions meaningful |
| Rate limiting | Can't spam actions | Patience timers help |

---

# 6. Ephemeral Rollups

**STATUS: Deferred to post-MVP**

- Need to study MagicBlock architecture more
- Should talk to MagicBlock team on Discord first
- MVP should prove game is FUN before optimizing speed
- Mainnet is fast enough for single-player MVP

---

# 10. MVP Scope

## Decisions

| Decision | Choice |
|----------|--------|
| MVP scope | **Playable Demo** |
| Facilities | **None in MVP** |
| Bot types | **Basic + VIP only** |
| Levels | **1-3** |
| Time estimate | **3-4 weeks** |

## MVP Features

| Feature | Description |
|---------|-------------|
| Lot creation | Player creates their parking lot |
| 2 starting spots | Begin with 2 regular spots |
| Basic bots | Spawn, wait, park, pay |
| VIP bots | Higher payout, lower patience |
| VIP spots | Unlock at level 2, multiplier bonus |
| 3 levels | Level 1 → 2 → 3 progression |
| $PARK token | Earn from bots, spend on upgrades |
| Treasury | Bot payments come from treasury |
| Patience timers | Bots leave if not assigned |
| Municipality approval | Request → wait → approved |
| Tutorial | First-time flow, guided first bot |

## MVP Progression

| Level | Spots | VIP Spots | Upgrade Cost | Wait Time |
|-------|-------|-----------|--------------|-----------|
| 1 | 2 | 0 | — | — |
| 2 | 4 | 1 | ~100 $PARK | 1-2 min |
| 3 | 6 | 1 | ~250 $PARK | 3-5 min |

## MVP Bot Types

| Bot Type | Patience | Payout | Spawn % |
|----------|----------|--------|---------|
| Basic | 60s | 1x | 80% |
| VIP | 10s | 5x | 20% |

## MVP Instructions (Program)

| Instruction | What it does |
|-------------|--------------|
| `initialize_game` | Create $PARK mint, treasury |
| `create_lot` | Player creates their lot (PDA) |
| `spawn_bot` | VRF spawns random bot into queue |
| `assign_bot` | Player assigns bot to spot |
| `checkout_bot` | Bot leaves, player gets paid |
| `request_upgrade` | Start municipality approval timer |
| `finalize_upgrade` | Complete upgrade after timer |

## What's NOT in MVP

| Feature | Version |
|---------|---------|
| Shop/Garage facilities | v2 |
| Shopper/Repair bots | v2 |
| SOL bonds | v2 |
| Monthly rent | v2 |
| Special events | v2+ |
| Multiplayer | v3+ |
| Ephemeral rollups | When needed |

## MVP Success Criteria

- [ ] Player can create a lot
- [ ] Bots spawn randomly (VRF)
- [ ] Player can assign bots to spots
- [ ] Bots park for duration, then checkout
- [ ] Player earns $PARK on checkout
- [ ] Player can upgrade from level 1 → 2 → 3
- [ ] VIP spots unlock at level 2
- [ ] VIP bots have higher payout + lower patience
- [ ] Tutorial guides first-time user
- [ ] All state is on-chain (Solana mainnet)
- [ ] Tests pass

## MVP Milestones

| Milestone | Description | Target |
|-----------|-------------|--------|
| M1 | Smart contract: lot + spots | Week 1 |
| M2 | Smart contract: bots + checkout | Week 2 |
| M3 | Smart contract: upgrades + economy | Week 3 |
| M4 | Testing + tuning + tutorial | Week 4 |

---

*Architecture complete. Ready to build.* 🚀
