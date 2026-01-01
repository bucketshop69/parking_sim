# 🅿️ Parking Sim Architecture & Design Document

> **Status:** Brainstorming Phase  
> **Last Updated:** December 25, 2025  
> **Authors:** Co-founders brainstorm session

---

## 🎯 The Vision (One-Liner)

**Clash of Clans meets Solana** — A parking lot management game where the backend IS the blockchain.

---

## 📋 Topics to Cover

We'll work through each of these systematically. Check them off as we solidify decisions.

| # | Topic | Status | Key Question |
|---|-------|--------|--------------|
| 1 | [Game Loop & Core Mechanics](#1-game-loop--core-mechanics) | ✅ Locked | What does the player actually DO? |
| 2 | [Player Progression](#2-player-progression) | ✅ Locked | What keeps them coming back? |
| 3 | [Bot System](#3-bot-system) | ✅ Locked | What makes bots interesting? |
| 4 | [Economy & Tokenomics](#4-economy--tokenomics) | ✅ Locked | Where does value come from and go? |
| 5 | [On-Chain vs Off-Chain](#5-on-chain-vs-off-chain) | ✅ Locked | What MUST be on-chain? |
| 6 | [Ephemeral Rollup Strategy](#6-ephemeral-rollup-strategy) | ⏸️ Deferred | When to delegate, when to commit? |
| 7 | [Multiplayer & Social](#7-multiplayer--social) | ⏸️ Out of Scope | How do players interact? |
| 8 | [Monetization](#8-monetization) | ⏸️ Out of Scope | How does this make money? |
| 9 | [Technical Stack](#9-technical-stack) | ⏸️ Out of Scope | What are we building with? |
| 10 | [MVP Scope](#10-mvp-scope) | ✅ Locked | What's v0.1? |

---

# Detailed Sections

---

## 1. Game Loop & Core Mechanics

### The Core Question
>
> What does a player DO in a 10-minute session?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| Bot spawning | **Time-based (offline OK)** | Bots accumulate based on real time, player doesn't need to be online |
| Player action | **Active assignment** | Player assigns bots to spots when they play |
| Overflow handling | **Patience Timer** | Bots wait, then leave if not assigned (missed revenue) |
| Skill expression | **Prioritization + Matching** | Who to take, where to put them |
| Depth model | **Time Pressure + Matching + Facility Limits** | Combines casual accessibility with skill ceiling |

### Core Mechanics — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     Parking Sim DEPTH MODEL                               │
│                                                                         │
│   LAYER 1: TIME PRESSURE                                                │
│   ─────────────────────                                                 │
│   • Different bot types have different PATIENCE                         │
│   • Different bot types have different PAYOUT                           │
│   • Bots leave if not assigned in time (missed revenue)                 │
│   • Player must prioritize: VIP (fast, high pay) vs Basic (slow, safe)  │
│                                                                         │
│   LAYER 2: MATCHING                                                     │
│   ────────────────────                                                  │
│   • Bots want different FACILITIES (garage, shop, etc.)                 │
│   • Correct facility match = bonus payout                               │
│   • No facility available = base pay only (or bot won't come)           │
│   • Facilities unlock at higher levels                                  │
│                                                                         │
│   LAYER 3: FACILITY CAPACITY                                            │
│   ──────────────────────────                                            │
│   • Facilities have limited throughput                                  │
│   • Garage bots take LONGER (more time in spot)                         │
│   • Must balance: quick turnover vs. high-value long stays              │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   RESULT: Two viable playstyles                                         │
│                                                                         │
│   🎮 CASUAL: Assign randomly, still earn, chill experience              │
│   🏆 SKILLED: Optimize assignments, earn 2-3x more, satisfying mastery  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Bot Type Framework — LOCKED 🔒

| Bot Type | Patience | Payout | Facility Needed | Stay Duration | Unlock Level |
|----------|----------|--------|-----------------|---------------|--------------|
| Basic | Long (60s) | 1x | None (just parking) | Short | 1 |
| Shopper | Medium (45s) | 1.5x | Shop | Medium | ? |
| Repair | Short (30s) | 2x | Garage | Long | ? |
| VIP | Very Short (10s) | 5x | Premium Spot? | Short | ? |

*Note: Exact numbers are placeholder — will tune during testing*

### Open Questions 🤔 (Remaining)

- [ ] What's the primary UI action? (Tap to assign? Drag & drop?)
- [ ] Exact spawn rates per level (need tuning)
- [ ] Do facilities have "inventory" that depletes? (e.g., shop items, garage parts)

### The Clash of Clans Parallel — ANSWERED ✅

| CoC | Parking Sim |
|-----|-----------|
| Collect gold/elixir | Collect parking fees (passive when bots leave) |
| Build defenses | Build lot upgrades + facilities |
| Attack other bases | **PRIORITIZE & MATCH bots** (active skill element) |
| Defend from attacks | **Manage patience timers** (don't let bots leave angry) |

### Notes & Discussion

#### 🆕 First-Time User Experience (FTUE) — The Onboarding Flow

**The Problem:** New players need $PARK to play, but can't earn $PARK without playing.

**The Solution:** The tutorial IS the faucet.

```
┌─────────────────────────────────────────────────────────────────┐
│                    FIRST-TIME USER FLOW                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. Player connects wallet (no game state detected)             │
│                         ↓                                       │
│  2. Tutorial triggered automatically                            │
│                         ↓                                       │
│  3. Player sees: 2 starter spots (guided, can't skip)           │
│                         ↓                                       │
│  4. Bot spawns → "Let this bot park in your lot" prompt         │
│                         ↓                                       │
│  5. Player assigns bot to spot (learns core action)             │
│                         ↓                                       │
│  6. Quick 30-second wait (maybe accelerated for tutorial)       │
│                         ↓                                       │
│  7. Bot leaves → Player receives first $PARK tokens! 🎉         │
│                         ↓                                       │
│  8. Progression pitch shown:                                    │
│     • "More cars parked → More $PARK earned"                    │
│     • "More $PARK → Apply for upgrades"                         │
│     • "Upgrades → More spots → More capacity"                   │
│                         ↓                                       │
│  9. Tutorial complete → Free play begins                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Insight:** Tutorial bots pay with "system-minted" $PARK — this is the only legitimate infinite faucet, and it's gated behind completing onboarding.

---

#### 🆕 The Core Loop (Post-Tutorial)

After tutorial ends, the main game loop begins:

```
     ┌──────────────────────────────────────────────────────────┐
     │                    THE CORE LOOP                         │
     │                                                          │
     │    ┌─────────┐                                           │
     │    │  BOTS   │ ←── Spawn rate depends on LOT LEVEL       │
     │    │ ARRIVE  │                                           │
     │    └────┬────┘                                           │
     │         ↓                                                │
     │    ┌─────────┐                                           │
     │    │ PLAYER  │ ←── Core action: ASSIGN bot to spot       │
     │    │ ASSIGNS │                                           │
     │    └────┬────┘                                           │
     │         ↓                                                │
     │    ┌─────────┐                                           │
     │    │  BOTS   │ ←── Duration varies by bot type           │
     │    │  PARK   │                                           │
     │    └────┬────┘                                           │
     │         ↓                                                │
     │    ┌─────────┐                                           │
     │    │  BOTS   │ ←── Pay $PARK → Player earns revenue      │
     │    │ LEAVE   │                                           │
     │    └────┬────┘                                           │
     │         ↓                                                │
     │    ┌─────────┐                                           │
     │    │ PLAYER  │ ←── Spend $PARK to unlock FACILITIES      │
     │    │UPGRADES │     (garage, shop, petrol pump, etc.)     │
     │    └────┬────┘                                           │
     │         │                                                │
     │         └──────────→ Loop continues...                   │
     │                                                          │
     └──────────────────────────────────────────────────────────┘
```

---

#### 🆕 Facilities Concept

**Progression isn't just "more spots" — it's unlocking FACILITIES that change gameplay:**

| Facility | What It Does | Unlocked At |
|----------|--------------|-------------|
| Basic Lot | Just parking spots | Start |
| Garage | Bots can get car repairs | Level ? |
| Shop | Bots can buy things | Level ? |
| Petrol Pump | Bots can refuel | Level ? |
| ??? | ??? | Level ? |

**Key Insight:** Different bots will WANT different facilities. A repair bot comes specifically for the garage. No garage? That bot won't come (or pays less?).

---

#### 🔍 DEEP DIVE NEEDED: Bot Spawn Rate by Level

**This is the core tuning lever. Let's figure it out.**

| Lot Level | Spots | Spawn Rate | Bots/Minute | Notes |
|-----------|-------|------------|-------------|-------|
| 1 | 2 | Every ??? sec | ? | Tutorial level |
| 2 | ? | Every ??? sec | ? | |
| 3 | ? | Every ??? sec | ? | |
| ... | ... | ... | ... | |

**Questions to answer:**

1. At Level 1 with 2 spots, how often should a bot arrive?
   - Too fast → Player overwhelmed, spots always full, bots leave angry
   - Too slow → Player bored, nothing to do

2. Does spawn rate increase with level, or stay same but CAPACITY increases?

3. What happens when all spots are full and a new bot arrives?
   - Bot waits in queue?
   - Bot leaves immediately (missed revenue)?
   - Bot leaves after X seconds patience?

---

## 2. Player Progression

### The Core Question
>
> What does a Day 1 lot vs Day 30 lot vs Day 100 lot look like?

### Decisions Made ✅

*(None yet)*

### Open Questions 🤔

- [ ] What are the upgrade tiers? (Levels? Unlocks? Both?)
- [ ] What are the upgrade costs? (Time? Money? Both?)
- [ ] Is there a "prestige" or reset mechanic?
- [ ] Are lots tradeable/sellable?
- [ ] Is there a level cap?

### Progression Systems to Consider

| System | Description | Pros | Cons |
|--------|-------------|------|------|
| Linear Levels | Lot Level 1 → 2 → 3... | Simple, clear goals | Can feel grindy |
| Tech Tree | Unlock branches (speed vs capacity) | Player choice, replayability | Complex to balance |
| Milestone Unlocks | Hit X revenue → unlock feature | Feels rewarding | Can gate content too hard |
| NFT Upgrades | Buy/mint upgrade NFTs | Real ownership | Pay-to-win concerns |

### Notes & Discussion

#### 🆕 Progression Philosophy — LOCKED 🔒

**Core Principle:** Progression should feel EARNED, not BOUGHT.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     DESIGN PHILOSOPHY                                   │
│                                                                         │
│   "Yes, this is Web3. Yes, players CAN buy $PARK."                     │
│   "But we don't DESIGN the game to reward that."                       │
│                                                                         │
│   The game is hand-crafted so:                                          │
│   • Skill and time matter more than wallet size                         │
│   • Each level feels intentionally designed                             │
│   • You can't just pay-to-skip the experience                          │
│                                                                         │
│   This differentiates Parking Sim from typical "pay-to-win" Web3 games.  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Implication:** We hand-tune every level. No formula. CoC-style designer control.

---

#### 🆕 Progression System — LOCKED 🔒

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| Progression type | **Infinite levels** | Like CoC, we add more over time |
| Level design | **Hand-crafted per level** | No formula, each level intentional |
| Upgrade mechanic | **Municipality Approval** | Request → Wait → Approved → Expand |
| Wait time scaling | **Variable by level** | Small upgrades fast, big upgrades slow |
| Facilities | **Separate unlocks with rent** | Pay to unlock, pay monthly to keep |
| Rent missed | **Soft punishment** | Facility "closed", bots can't use it, base pay only |

---

#### 🆕 Municipality Approval System

```
┌─────────────────────────────────────────────────────────────────┐
│                   UPGRADE FLOW                                   │
│                                                                  │
│   1. Player reaches requirements (enough $PARK, enough revenue) │
│                         ↓                                        │
│   2. Player submits "Request for Upgrade" to Municipality        │
│                         ↓                                        │
│   3. Wait time begins (varies by upgrade size)                   │
│      • Small upgrade (more spots): Minutes                       │
│      • Big upgrade (new facility): Hours                         │
│                         ↓                                        │
│   4. Approval granted → Player pays $PARK → Upgrade applied     │
│                                                                  │
│   THEMATIC: You're running a real business. You need permits!   │
└─────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 Facility Rental System

```
┌─────────────────────────────────────────────────────────────────┐
│                   FACILITY LIFECYCLE                             │
│                                                                  │
│   UNLOCK                                                         │
│   └─→ Pay one-time unlock fee                                   │
│   └─→ Facility becomes available                                │
│                                                                  │
│   ACTIVE                                                         │
│   └─→ Pay monthly rent in $PARK                                 │
│   └─→ Facility-specific bots can use it                         │
│   └─→ Player earns premium rates                                │
│                                                                  │
│   RENT MISSED (soft punishment)                                  │
│   └─→ Facility marked "CLOSED"                                  │
│   └─→ Bots still spawn wanting that facility                    │
│   └─→ But bots CAN'T use it → pay base rate only               │
│   └─→ Player feels the loss without losing bots entirely        │
│   └─→ Pay overdue rent anytime to reopen                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

#### 📋 Level Design — TO BE DESIGNED

**This is a separate design task. We'll hand-craft each level.**

**What we need to define per level:**

| Field | Description |
|-------|-------------|
| Level Number | 1, 2, 3, ... |
| Spots | How many parking spots |
| Facility Unlock | Does this level unlock a facility? |
| Upgrade Cost | $PARK required |
| Wait Time | Municipality approval duration |
| Monthly Rent | If facility, ongoing cost |
| Spawn Rate | Bots per minute at this level |
| Bot Types Available | Which bots can appear |
| Income Potential | Expected $PARK per session |

**Status:** ⏳ Deferred to Level Design phase (after architecture complete)

---

## 3. Bot System

### The Core Question
>
> What makes each bot spawn exciting/interesting?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| Bot spawn gating | **Level + Facility (AND)** | Need both level AND facility for advanced bots |
| Spot types | **Regular + VIP** | VIP spots have multiplier bonus |
| VIP spot progression | **Level milestone** | Fixed per level (e.g., Level 5 = 1 VIP spot) |
| Bonus system | **Adaptive** | Flexible multipliers for events/tuning |
| Special/Event bots | **Post-MVP** | Keep simple with 4 types for now |

### Notes & Discussion

#### 🆕 Bot Lifecycle — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         BOT LIFECYCLE                                   │
│                                                                         │
│   PHASE 1: ARRIVAL                                                      │
│   ─────────────────                                                     │
│   • Bot spawns (random type based on level + facilities available)     │
│   • Bot enters WAITING QUEUE                                           │
│   • Patience timer starts                                               │
│                                                                         │
│   PHASE 2: WAITING                                                      │
│   ────────────────                                                      │
│   • Bot waits for player to assign                                     │
│   • Timer ticking down                                                  │
│   • If timer hits 0 → Bot leaves ANGRY (missed revenue)               │
│                                                                         │
│   PHASE 3: ASSIGNED                                                     │
│   ─────────────────                                                     │
│   • Player assigns bot to spot                                         │
│   • Bot moves to spot                                                   │
│   • Stay duration timer starts                                         │
│   • If facility bot → uses facility (or base pay if closed)           │
│                                                                         │
│   PHASE 4: PARKED                                                       │
│   ────────────────                                                      │
│   • Bot occupies spot                                                   │
│   • Timer counting down                                                 │
│   • Spot is BLOCKED (can't assign another bot here)                   │
│                                                                         │
│   PHASE 5: CHECKOUT                                                     │
│   ─────────────────                                                     │
│   • Stay timer complete                                                 │
│   • Bot pays $PARK (from treasury to player)                          │
│   • Payout calculated with all multipliers                             │
│   • Bot leaves → Spot is FREE again                                   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 Bot Spawn Logic — LOCKED 🔒

**Bots spawn based on: Level AND Facilities (both required)**

| Bot Type | Level Required | Facility Required | Spawns If... |
|----------|----------------|-------------------|--------------|
| Basic | 1+ | None | Always |
| Shopper | 5+ | Shop unlocked | Level ≥ 5 AND has Shop |
| Repair | 8+ | Garage unlocked | Level ≥ 8 AND has Garage |
| VIP | 3+ | None (needs VIP spot) | Level ≥ 3 (VIP spots exist) |

*Note: Level numbers are placeholder — will tune during level design*

---

#### 🆕 Spot Types — LOCKED 🔒

**Not all spots are equal:**

| Spot Type | Multiplier | Unlock |
|-----------|------------|--------|
| Regular | 1x | Default |
| VIP | Higher (adaptive) | Level milestone |

**VIP Spot Progression (example):**

| Level | Total Spots | VIP Spots |
|-------|-------------|-----------|
| 1-2 | 2-4 | 0 |
| 3-4 | 6-8 | 1 |
| 5-6 | 10-12 | 2 |
| 7+ | 14+ | 3+ |

*Exact numbers in Level Design doc*

---

#### 🆕 Payout Formula — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         PAYOUT CALCULATION                              │
│                                                                         │
│   PAYOUT = Base_Rate × Bot_Multiplier × Spot_Multiplier × Facility_Bonus│
│                                                                         │
│   BASE RATE                                                             │
│   └─→ Fixed amount per checkout (e.g., 10 $PARK)                       │
│   └─→ May scale with level (higher levels = higher base)               │
│                                                                         │
│   BOT MULTIPLIER                                                        │
│   └─→ Basic: 1.0x                                                       │
│   └─→ Shopper: 1.5x                                                     │
│   └─→ Repair: 2.0x                                                      │
│   └─→ VIP: 5.0x                                                         │
│                                                                         │
│   SPOT MULTIPLIER                                                       │
│   └─→ Regular: 1.0x                                                     │
│   └─→ VIP Spot: 1.5x (adaptive, can change for events)                 │
│                                                                         │
│   FACILITY BONUS                                                        │
│   └─→ No facility used: 1.0x                                           │
│   └─→ Shopper uses Shop: +0.3x (adaptive)                              │
│   └─→ Repair uses Garage: +0.3x (adaptive)                             │
│   └─→ Facility closed: 1.0x (no bonus)                                 │
│                                                                         │
│   ADAPTIVE = Can be tuned for events, promotions, balancing            │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   EXAMPLE CALCULATIONS                                                  │
│                                                                         │
│   Basic + Regular + No facility                                         │
│   = 10 × 1.0 × 1.0 × 1.0 = 10 $PARK                                   │
│                                                                         │
│   VIP + VIP Spot + No facility                                         │
│   = 10 × 5.0 × 1.5 × 1.0 = 75 $PARK                                   │
│                                                                         │
│   Shopper + Regular + Uses Shop                                         │
│   = 10 × 1.5 × 1.0 × 1.3 = 19.5 $PARK                                 │
│                                                                         │
│   VIP + VIP Spot + Uses Shop (if VIP shops)                            │
│   = 10 × 5.0 × 1.5 × 1.3 = 97.5 $PARK                                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 Bot Types Summary — LOCKED 🔒

| Bot Type | Patience | Payout Multi | Facility | Stay Duration | Risk/Reward |
|----------|----------|--------------|----------|---------------|-------------|
| Basic | 60s | 1x | None | Short | Low/Low |
| Shopper | 45s | 1.5x | Shop | Medium | Med/Med |
| Repair | 30s | 2x | Garage | Long | Med/High |
| VIP | 10s | 5x | VIP Spot | Short | High/High |

---

#### 🔮 Future (Post-MVP)

| Feature | Description |
|---------|-------------|
| Special Event Bots | Holiday bots, limited time, unique rewards |
| Lucky Bot | Rare spawn, huge payout |
| Group Bookings | Multiple bots arrive together |
| Bot Personalities | Visual variety, different cars |

---

## 4. Economy & Tokenomics

### The Core Question
>
> Where does $PARK come from, where does it go, and why does it have value?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| Token supply | **Fixed cap (100M $PARK)** | Scarcity, no infinite inflation |
| Mint authority | **Program-controlled** | Not a wallet, rules enforced by code |
| Bot payments | **From treasury** | Simple, controlled distribution |
| Sink destination | **Back to treasury** | Circular economy, sustainable |
| SOL bonds | **Lock-only (no risk)** | Simple for MVP, get SOL back on close |
| DEX management | **Not our problem** | Focus on in-game economy, market does its thing |
| Dual currency | **$PARK (soft) + SOL (hard)** | Earn $PARK, lock SOL for premium |

### The Fundamental Tension

**Traditional games (CoC):**

- Gems are bought with real money (USD → Gems)
- Gems are infinite (Supercell mints whenever someone buys)
- No way to cash out (one-way door)
- Economy is "closed" — devs control everything

**Blockchain games:**

- Tokens CAN be traded (two-way door)
- If infinite mint → token goes to zero
- Need real demand/utility to maintain value
- Economy is "open" — market forces apply

**Parking Sim approach:** Fixed supply, circular economy, let DEX happen naturally.

### Notes & Discussion

#### 🆕 MVP Tokenomics — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    $PARK TOKEN ECONOMY                                  │
│                                                                         │
│   SUPPLY                                                                │
│   ──────                                                                │
│   • Total Cap: 100,000,000 $PARK (fixed, enforced by program)          │
│   • Mint authority: Program-controlled (not a wallet)                  │
│   • Once cap hit → no more minting, ever                               │
│                                                                         │
│   DISTRIBUTION                                                          │
│   ────────────                                                          │
│   • Treasury: 80% (80M) — for bot payouts                              │
│   • Team: 10% (10M) — for development                                  │
│   • Reserve: 10% (10M) — for future features, partnerships             │
│                                                                         │
│   FAUCETS (how $PARK enters player wallets)                            │
│   ─────────────────────────────────────────                            │
│   • Bot checkout payments (from treasury)                              │
│   • Tutorial completion (one-time, small amount)                       │
│                                                                         │
│   SINKS (how $PARK leaves player wallets)                              │
│   ───────────────────────────────────────                              │
│   • Level upgrade costs → back to treasury                             │
│   • Facility unlock fees → back to treasury                            │
│   • Monthly rent payments → back to treasury                           │
│                                                                         │
│   CIRCULAR FLOW                                                         │
│   ─────────────                                                         │
│                                                                         │
│      ┌─────────────┐                                                   │
│      │  TREASURY   │◄────────────────────────────┐                     │
│      │  (80M $PARK)│                             │                     │
│      └──────┬──────┘                             │                     │
│             │                                    │                     │
│             │ Bots pay players                   │                     │
│             ↓                                    │                     │
│      ┌─────────────┐                             │                     │
│      │   PLAYERS   │                             │                     │
│      │   WALLETS   │─────────────────────────────┘                     │
│      └─────────────┘  Players spend on upgrades,                       │
│                       facilities, rent                                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 SOL Bonds — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    SOL BOND SYSTEM                                      │
│                                                                         │
│   PURPOSE                                                               │
│   • Premium facilities require SOL bond (not $PARK)                    │
│   • Creates "skin in the game" with real value                         │
│   • Players don't "lose" SOL — it's locked, not spent                  │
│                                                                         │
│   HOW IT WORKS                                                          │
│   ─────────────                                                         │
│                                                                         │
│   OPEN FACILITY                                                         │
│   └─→ Player locks X SOL as bond                                       │
│   └─→ SOL held in program escrow                                       │
│   └─→ Facility becomes active                                          │
│                                                                         │
│   FACILITY ACTIVE                                                       │
│   └─→ SOL remains locked                                               │
│   └─→ Player earns premium $PARK from facility bots                    │
│                                                                         │
│   CLOSE FACILITY                                                        │
│   └─→ Player requests close                                            │
│   └─→ SOL bond returned in full                                        │
│   └─→ Facility deactivates                                             │
│                                                                         │
│   MVP: Lock-only, no penalty for closing                               │
│   FUTURE: Maybe add slashing for abandoned facilities?                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 DEX Philosophy — LOCKED 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    DEX STANCE                                           │
│                                                                         │
│   WE CONTROL                        WE DON'T CONTROL                   │
│   ──────────                        ──────────────────                  │
│   • Total supply                    • DEX listings                     │
│   • Mint authority                  • Trading price                    │
│   • How players earn $PARK          • Speculation                      │
│   • How players spend $PARK         • Liquidity pools                  │
│   • In-game economy balance         • External market dynamics         │
│                                                                         │
│   PHILOSOPHY                                                            │
│   ──────────                                                            │
│   "We make the game economy fun and balanced.                          │
│    If someone creates a DEX pool, that's their business.               │
│    We don't encourage or discourage trading.                           │
│    We focus on the GAME, not the SPECULATION."                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🔮 Future Considerations (Post-MVP)

| Feature | Description | When |
|---------|-------------|------|
| DeFi Boosts | Lock JUP/LSTs for in-game bonuses | v2+ |
| SOL Bond Yield | Stake locked SOL, yield funds treasury | v2+ |
| Deflationary Burns | Burn % of sinks instead of recycling | If inflation issues |
| Token Buybacks | Use revenue to buy $PARK from market | If we monetize |

---

## 5. On-Chain vs Off-Chain

### The Core Question
>
> What MUST be on Solana vs what can be in a normal database?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| MVP approach | **Everything on Mainnet** | Start simple, optimize later |
| Ephemeral rollups | **Future (post-MVP)** | Need to study MagicBlock more |
| VRF | **On-chain (mainnet)** | Provably random, no shortcuts |

### Notes & Discussion

#### 🆕 MVP Architecture — LOCKED 🔒

**Philosophy: Start on mainnet. Optimize later.**

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    MVP ARCHITECTURE (SIMPLE)                            │
│                                                                         │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │                     SOLANA MAINNET                               │  │
│   │                     (Everything important)                       │  │
│   │                                                                  │  │
│   │   • $PARK token + treasury                                      │  │
│   │   • SOL bonds (locked)                                          │  │
│   │   • Lot state (level, spots, facilities)                        │  │
│   │   • Bot spawns (VRF)                                            │  │
│   │   • Bot assignments                                              │  │
│   │   • Parking/checkout transactions                                │  │
│   │   • Upgrades and facility unlocks                               │  │
│   │   • Revenue collection                                           │  │
│   │                                                                  │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                              ↑↓                                        │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │                     CLIENT (Off-chain)                          │  │
│   │                     (UI/UX only)                                 │  │
│   │                                                                  │  │
│   │   • Graphics / animations                                        │  │
│   │   • Sound effects                                                │  │
│   │   • UI state (menus, buttons)                                   │  │
│   │   • Local caching (read optimization)                           │  │
│   │                                                                  │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🆕 What Goes On-Chain — LOCKED 🔒

| Data/Action | On-Chain | Reasoning |
|-------------|----------|-----------|
| $PARK token balances | ✅ Yes | Real value, must be trustless |
| $PARK treasury | ✅ Yes | Controls economy |
| SOL bonds | ✅ Yes | Real SOL at stake |
| Lot ownership | ✅ Yes | Must be verifiable |
| Lot state (level, spots) | ✅ Yes | Permanent progression |
| Facility state | ✅ Yes | Unlocks, rent status |
| Bot spawns (VRF) | ✅ Yes | Must be provably random |
| Bot assignments | ✅ Yes | Game actions |
| Parking transactions | ✅ Yes | State changes |
| Checkout/payments | ✅ Yes | $PARK movement |
| Upgrades | ✅ Yes | State changes |

---

#### 🆕 What Stays Off-Chain — LOCKED 🔒

| Data/Action | Off-Chain | Reasoning |
|-------------|-----------|-----------|
| UI state | ✅ Yes | No value, just display |
| Animations | ✅ Yes | Visual only |
| Sound | ✅ Yes | No blockchain needed |
| Local cache | ✅ Yes | Performance optimization |
| Analytics | ✅ Yes | Optional tracking |

---

#### ⚠️ MVP Tradeoffs (Acknowledged)

**Putting everything on mainnet means:**

| Tradeoff | Impact | Mitigation |
|----------|--------|------------|
| Slower transactions | ~400ms per action | Acceptable for MVP |
| Transaction costs | SOL fees per action | Keep actions meaningful, batch if possible |
| Rate limiting | Can't spam actions | Design around it (patience timers help!) |

**This is FINE for MVP.** We're proving the game is fun, not optimizing for 1000 concurrent users.

---

#### 🔮 Future Optimization (Post-MVP)

| Technology | What It Enables | When to Add |
|------------|-----------------|-------------|
| MagicBlock Ephemeral Rollups | Fast gameplay, cheap transactions | When we need speed |
| Account delegation | Move game state to rollup | After studying MagicBlock |
| BOLT ECS | Entity-component system for games | If game gets complex |
| Clockwork/Automation | Scheduled actions (rent due, etc.) | When needed |

**Action item:** Talk to MagicBlock Discord about Parking Sim use case before implementing.

---

## 6. Ephemeral Rollup Strategy

### The Core Question
>
> How do we use MagicBlock to make gameplay fast while keeping value secure?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| MVP approach | **Skip for MVP** | Start on mainnet, optimize later |
| When to add | **Post-MVP** | After studying MagicBlock, talking to team |

### Notes & Discussion

#### 🆕 Status — DEFERRED TO POST-MVP 🔒

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    EPHEMERAL ROLLUPS                                    │
│                                                                         │
│   STATUS: Not needed for MVP                                           │
│                                                                         │
│   WHY DEFER:                                                            │
│   • Need to study MagicBlock architecture more                         │
│   • Should talk to MagicBlock team on Discord first                    │
│   • MVP should prove game is FUN before optimizing speed               │
│   • Mainnet is fast enough for single-player MVP                       │
│                                                                         │
│   WHEN TO REVISIT:                                                      │
│   • If mainnet speed becomes a problem                                 │
│   • If transaction costs become prohibitive                            │
│   • If we add real-time multiplayer                                    │
│   • After MVP launch and user feedback                                 │
│                                                                         │
│   ACTION ITEMS:                                                         │
│   □ Study MagicBlock documentation                                     │
│   □ Join MagicBlock Discord                                            │
│   □ Ask about Parking Sim use case                                       │
│   □ Understand delegation/commit patterns                              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### 🔮 Future Architecture (When Ready)

When we DO add ephemeral rollups, the architecture would be:

```
MAINNET (Source of Truth)     ROLLUP (Fast Gameplay)
━━━━━━━━━━━━━━━━━━━━━━━━     ━━━━━━━━━━━━━━━━━━━━━━
• $PARK balances              • Bot spawns
• SOL bonds                   • Bot queue
• Lot ownership               • Assignments
• Upgrade state               • Parking timers
                              • Session revenue
        ↑                            │
        └────── commit() ────────────┘
```

**But that's a problem for future us.**

---
[Space for our discussion notes]

```

---

## 7. Multiplayer & Social

### The Core Question
>
> How do players interact with each other?

### Decisions Made ✅

*(None yet)*

### Multiplayer Models

| Model | Description | Complexity |
|-------|-------------|------------|
| Solo Only | Your lot, your game | Low |
| Leaderboards | Compete on metrics | Low |
| Trading | Exchange items/$PARK | Medium |
| Visiting | See other lots | Medium |
| Cooperative | Share lots, combine revenue | High |
| Competitive | Fight for bots, sabotage | High |

### Open Questions 🤔

- [ ] Can players visit each other's lots?
- [ ] Is there trading between players?
- [ ] Are there guilds/clans?
- [ ] Is there PvP of any kind?
- [ ] Global chat? Friends list?

### Notes & Discussion

```

[Space for our discussion notes]

```

---

## 8. Monetization

### The Core Question
>
> How does Parking Sim make money?

### Decisions Made ✅

*(None yet)*

### Revenue Models

| Model | How It Works | Player Perception |
|-------|--------------|-------------------|
| Token Sale | Sell $PARK directly | "ICO vibes" |
| NFT Sales | Sell lot NFTs, cosmetics | Acceptable if fair |
| Transaction Fees | % cut of all $PARK moves | Hidden, sustainable |
| Premium Features | Pay for upgrades, speed-ups | CoC model, proven |
| Ads | Show ads for rewards | Works but feels cheap |
| Season Pass | Pay for exclusive content | Works if content is good |

### Open Questions 🤔

- [ ] Is there an upfront cost to play?
- [ ] Is there a free tier?
- [ ] Where does the treasury come from?
- [ ] What's the team's cut on transactions?

### Notes & Discussion

```

[Space for our discussion notes]

```

---

## 9. Technical Stack

### The Core Question
>
> What tools and frameworks are we building with?

### Current Stack (from training arc)

| Layer | Technology |
|-------|------------|
| Smart Contracts | Anchor (Rust) |
| Blockchain | Solana |
| Fast Execution | MagicBlock Ephemeral Rollups |
| Randomness | VRF (MagicBlock/ORAO) |
| Tokens | SPL Token Program |
| Testing | Anchor Tests (TypeScript) |

### Still Need to Decide

| Layer | Options | Decision |
|-------|---------|----------|
| Frontend | React? Unity? Godot? | ⬜ |
| State Management | ? | ⬜ |
| Backend (off-chain) | Node? Rust? None? | ⬜ |
| Database | Postgres? None? | ⬜ |
| Hosting | Vercel? AWS? | ⬜ |
| Indexing | Helius? TheGraph? Custom? | ⬜ |

### Notes & Discussion

```

[Space for our discussion notes]

```

---

## 10. MVP Scope

### The Core Question
>
> What's the smallest thing we can ship that proves the concept?

### Decisions Made ✅

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| MVP scope | **Option B: Playable Demo** | Proves progression is fun, not just that it works |
| Facilities | **None in MVP** | Add in v2 |
| Bot types | **Basic + VIP only** | Enough for strategy |
| Levels | **1-3** | Enough to feel progression |

### Notes & Discussion

#### 🆕 MVP Definition — LOCKED 🔒

```

┌─────────────────────────────────────────────────────────────────────────┐
│                         MVP: PLAYABLE DEMO                              │
│                                                                         │
│   GOAL: Prove the game is FUN and progression feels good               │
│   TIME ESTIMATE: 3-4 weeks                                              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘

```

---

#### 🆕 MVP Features — MUST HAVE ✅

| Feature | Description | Why MVP |
|---------|-------------|---------|
| **Lot creation** | Player creates their parking lot | Core |
| **2 starting spots** | Begin with 2 regular spots | Core |
| **Basic bots** | Spawn, wait, park, pay | Core loop |
| **VIP bots** | Higher payout, lower patience | Strategy |
| **VIP spots** | Unlock at level 2, multiplier bonus | Progression |
| **3 levels** | Level 1 → 2 → 3 progression | Proves progression |
| **$PARK token** | Earn from bots, spend on upgrades | Economy |
| **Treasury** | Bot payments come from treasury | Economy |
| **Patience timers** | Bots leave if not assigned | Tension |
| **Municipality approval** | Request upgrade → wait → approved | Progression feel |
| **Tutorial** | First-time flow, guided first bot | Onboarding |

---

#### 🆕 MVP Progression — LOCKED 🔒

| Level | Spots | VIP Spots | Upgrade Cost | Wait Time |
|-------|-------|-----------|--------------|-----------|
| 1 | 2 | 0 | — | — |
| 2 | 4 | 1 | ~100 $PARK | 1-2 min |
| 3 | 6 | 1 | ~250 $PARK | 3-5 min |

*Exact numbers to tune during playtesting*

---

#### 🆕 MVP Bot Types — LOCKED 🔒

| Bot Type | Patience | Payout | Spawn % | Notes |
|----------|----------|--------|---------|-------|
| Basic | 60s | 1x | 80% | Reliable, safe |
| VIP | 10s | 5x | 20% | High risk/reward |

---

#### 🆕 MVP Economy — LOCKED 🔒

```

FAUCETS                          SINKS
─────────                        ─────
• Bot checkout (from treasury)   • Level upgrades (to treasury)
• Tutorial reward

FLOW:
Treasury → Bot pays player → Player upgrades → Treasury
                ↑                                  │
                └──────────────────────────────────┘

```

---

#### 🆕 MVP Instructions (Program) — LOCKED 🔒

| Instruction | What it does |
|-------------|--------------|
| `initialize_game` | Create $PARK mint, treasury |
| `create_lot` | Player creates their lot (PDA) |
| `spawn_bot` | VRF spawns random bot into queue |
| `assign_bot` | Player assigns bot to spot |
| `checkout_bot` | Bot leaves, player gets paid |
| `request_upgrade` | Start municipality approval timer |
| `finalize_upgrade` | Complete upgrade after timer |

---

#### 🆕 What's NOT in MVP — DEFERRED 🔒

| Feature | Reason | Version |
|---------|--------|---------|
| Shop facility | Adds complexity | v2 |
| Garage facility | Adds complexity | v2 |
| Shopper bots | Need Shop first | v2 |
| Repair bots | Need Garage first | v2 |
| SOL bonds | Can add later | v2 |
| Monthly rent | Can add later | v2 |
| Special events | Polish feature | v2+ |
| Multiplayer | Out of scope | v3+ |
| Ephemeral rollups | Optimization | When needed |

---

#### 🆕 MVP Success Criteria

**We know MVP is done when:**

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

---

#### 🆕 MVP Milestones

| Milestone | Description | Target |
|-----------|-------------|--------|
| M1 | Smart contract: lot + spots | Week 1 |
| M2 | Smart contract: bots + checkout | Week 2 |
| M3 | Smart contract: upgrades + economy | Week 3 |
| M4 | Testing + tuning + tutorial | Week 4 |

---

# 📝 Session Notes

## Session 1 — December 25, 2025

### Key Decisions

- ✅ Game mode: **Active** (player must be online)
- ✅ Approach: Treat like "Clash of Clans with Solana backend"

### Key Insights

- Player has completed training arc projects 1-5
- Confidence went from 3/10 to ready for full project
- Want to document for "building in public" marketing

### Next Steps

- [ ] Work through Topic 1 (Game Loop) in detail
- [ ] Define the core 10-minute play session

### Open Threads

- Economy design needs deep dive (acknowledged knowledge gap)
- Need to map CoC-style loop to parking mechanics

---

# 🎬 Marketing Assets Tracker

## Video Content Ideas

| Topic | Hook | Status |
|-------|------|--------|
| "Building Clash of Clans on Solana" | The vision pitch | ⬜ Not filmed |
| "Why On-Chain Games?" | Educational + vision | ⬜ Not filmed |
| "Designing a Token Economy" | Behind the scenes | ⬜ Not filmed |
| "From Zero to Solana Dev" | Personal journey | ⬜ Not filmed |

## Tweetable Moments

```

"What if Clash of Clans was built on Solana?"

"I went from 3/10 in Anchor to building a full game. Here's the training arc that got me there 🧵"

"The backend of your game IS the blockchain. Here's what that actually means:"

```

---

*This document is a living artifact. We'll update it as we make decisions.*
