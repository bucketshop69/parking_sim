# 🅿️ Parking Sim Technical Architecture

> **Status:** MVP Design  
> **Last Updated:** December 27, 2025  
> **Scope:** Levels 1-3

---

## 🎯 The Game

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           THE GAME                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  You run a parking lot. Bots are your "regulars" - local residents      │
│  who come to park. They have their own schedules. They show up,         │
│  wait for you to assign them a spot, and leave when done.               │
│                                                                         │
│  If you're not around, they still come. They still wait. They still     │
│  leave if you ignore them too long. Life goes on.                       │
│                                                                         │
│  Your best customers eventually become VIPs.                            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 📈 Progression (Levels 1-3)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  LEVEL 1                          LEVEL 2                    LEVEL 3    │
│  ────────                         ────────                   ────────   │
│  5 spots                          7 spots                    10 spots   │
│  5 regulars (all Basic)           7 regulars (1 VIP)         10 regulars│
│                                                              (2 VIPs?)  │
│        │                                │                               │
│        │ 100 $PARK + 30 min wait        │ 200 $PARK + 45 min wait       │
│        ▼                                ▼                               │
│                                                                         │
│  Basic bot: 5 $PARK, 60s patience                                       │
│  VIP bot: 25 $PARK, 10s patience (promoted from top spender)            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Progression Table

| Level | Spots | Bots | VIPs | Upgrade Cost | Wait Time |
|-------|-------|------|------|--------------|-----------|
| 1 | 5 | 5 | 0 | — | — |
| 2 | 7 | 7 | 1 | 100 $PARK | 30 min |
| 3 | 10 | 10 | 2? | 200 $PARK | 45 min |

### Bot Economics

| Bot Type | Payout | Patience | How to Get |
|----------|--------|----------|------------|
| Basic | 5 $PARK | 60 seconds | Default |
| VIP | 25 $PARK | 10 seconds | Top spender gets promoted on level up |

---

## 🤖 The Bots (Regulars)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  BOTS ARE PERSISTENT CHARACTERS                                         │
│                                                                         │
│  Each bot is a "person" who:                                            │
│  • Has their own account on-chain                                       │
│  • Remembers how many times they've visited                             │
│  • Remembers how much $PARK they've paid you                            │
│  • Remembers how many times you ignored them                            │
│  • Has a scheduled "next arrival" time                                  │
│                                                                         │
│  Top spender at end of level → Gets promoted to VIP next level          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Bot Lifecycle

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│    ┌──────┐                                                             │
│    │ IDLE │  Bot is "at home", has a scheduled arrival time             │
│    └───┬──┘                                                             │
│        │                                                                │
│        │ arrival time reached                                           │
│        ▼                                                                │
│   ┌─────────┐                                                           │
│   │ WAITING │  Bot is at your lot, patience timer ticking               │
│   └────┬────┘                                                           │
│        │                                                                │
│   ┌────┴────┐                                                           │
│   │         │                                                           │
│   ▼         ▼                                                           │
│ Player    Patience                                                      │
│ assigns   runs out                                                      │
│   │         │                                                           │
│   ▼         ▼                                                           │
│ ┌──────┐  ┌────────┐                                                    │
│ │PARKED│  │ MISSED │  Left angry, "times_missed" goes up                │
│ └───┬──┘  └───┬────┘                                                    │
│     │         │                                                         │
│     │ stay    │                                                         │
│     │ ends    │                                                         │
│     ▼         │                                                         │
│ ┌──────┐      │                                                         │
│ │ PAID │      │  You get $PARK, bot's "total_paid" goes up              │
│ └───┬──┘      │                                                         │
│     │         │                                                         │
│     └────┬────┘                                                         │
│          │                                                              │
│          │ schedule next arrival (30-90 sec random)                     │
│          ▼                                                              │
│       ┌──────┐                                                          │
│       │ IDLE │  Cycle repeats                                           │
│       └──────┘                                                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## ⏰ The Event Queue (The Magic)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  HOW BOTS "LIVE THEIR LIVES" WITHOUT A SERVER                           │
│                                                                         │
│  We store a list of FUTURE EVENTS:                                      │
│                                                                         │
│    "Bot 2 arrives at 10:00:45"                                          │
│    "Bot 0 finishes parking at 10:01:12"                                 │
│    "Bot 3 arrives at 10:01:30"                                          │
│    "Bot 2 loses patience at 10:01:45"                                   │
│    ...                                                                  │
│                                                                         │
│  When player does ANYTHING (assign, checkout, refresh):                 │
│                                                                         │
│    1. Look at the queue                                                 │
│    2. Process everything that should have happened by now               │
│    3. Then do what the player asked                                     │
│                                                                         │
│  RESULT: Bots feel alive. They came, waited, left, paid — all           │
│          "happened" even though player wasn't there.                    │
│          We just CALCULATE it when they return.                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Event Types

| Event | What It Means | What Happens When Processed |
|-------|---------------|----------------------------|
| ARRIVAL | Bot shows up at lot | Bot → Waiting, schedule PATIENCE event |
| PATIENCE | Bot's patience expires | If still Waiting → Missed, schedule next ARRIVAL |
| CHECKOUT | Bot's parking time done | Pay player, Bot → Idle, schedule next ARRIVAL |

### Example Timeline

```
Time: 10:00:00 - Player initializes lot
──────────────────────────────────────
Queue created:
  [ARRIVAL bot:0 at:10:00:00]  ← immediate
  [ARRIVAL bot:1 at:10:00:52]
  [ARRIVAL bot:2 at:10:01:38]
  [ARRIVAL bot:3 at:10:02:15]
  [ARRIVAL bot:4 at:10:03:01]

Bot 0 immediately arrives (Waiting)
Queue adds: [PATIENCE bot:0 at:10:01:00]


Time: 10:00:30 - Player assigns bot 0 to spot 1
─────────────────────────────────────────────────
STEP 1: Process queue
  → Nothing due yet (bot 1 arrives at 10:00:52)
  
STEP 2: Do assign
  → Bot 0: Waiting → Parked
  → Remove PATIENCE event for bot 0
  → Add: [CHECKOUT bot:0 at:10:01:00] (30s stay)


Time: 10:03:00 - Player comes back, does any action
─────────────────────────────────────────────────────
STEP 1: Process queue (lots happened!)
  
  → 10:00:52: ARRIVAL bot:1 → Waiting
  → 10:01:00: CHECKOUT bot:0 → Paid! 5 $PARK → Idle → schedule next arrival
  → 10:01:38: ARRIVAL bot:2 → Waiting
  → 10:01:52: PATIENCE bot:1 expired → Missed! → Idle → schedule next
  → 10:02:15: ARRIVAL bot:3 → Waiting
  → 10:02:38: PATIENCE bot:2 expired → Missed! → Idle → schedule next
  → 10:03:01: ARRIVAL bot:4 → Waiting
  
  Current state after processing:
  • Bot 0: Idle (paid, rescheduled)
  • Bot 1: Idle (missed, rescheduled)
  • Bot 2: Idle (missed, rescheduled)
  • Bot 3: Waiting (patience ticking)
  • Bot 4: Waiting (just arrived)

Player sees: "You earned 5 $PARK while away! 2 customers left angry."
```

---

## 💾 Data (What Lives On-Chain)

### 1. Lot Account (One per player)

| Field | Type | Description |
|-------|------|-------------|
| owner | Pubkey | Player's wallet |
| level | u8 | 1, 2, or 3 |
| total_earned | u64 | Lifetime $PARK earned |
| bot_count | u8 | Number of bots (5 → 7 → 10) |
| spots | [SpotState; 10] | Empty or occupied by which bot |
| event_queue | Vec<Event> | The scheduler (sorted by time) |
| upgrade_requested_at | i64 | When upgrade was requested (0 if not) |
| upgrade_paid | bool | Whether $PARK was paid |

### 2. Bot Account (5-10 per lot)

| Field | Type | Description |
|-------|------|-------------|
| lot | Pubkey | Which lot this bot belongs to |
| index | u8 | Bot #0, #1, #2, etc. |
| bot_type | u8 | 0 = Basic, 1 = VIP |
| total_visits | u32 | Lifetime successful parkings |
| total_paid | u64 | Lifetime $PARK paid to this lot |
| times_missed | u32 | Times left angry (patience expired) |
| status | u8 | Idle, Waiting, or Parked |
| spot_index | u8 | Which spot (if Parked) |

*Note: Arrival times, patience deadlines, checkout times are stored in the Event Queue, not on the bot.*

### 3. Treasury Account (One global)

| Field | Type | Description |
|-------|------|-------------|
| authority | Pubkey | Program authority |
| balance | u64 | $PARK available for payouts |

---

## 🎮 Actions (Instructions)

### 1. initialize_lot

**What it does:**

- Creates the player's lot account
- Creates 5 bot accounts (Level 1 regulars)
- Schedules first bot to arrive immediately
- Schedules other 4 bots with random arrival times (30-90 sec apart)

**When called:** Player starts game for the first time

---

### 2. assign_bot

**What it does:**

1. **Process event queue** (catch up on everything that happened)
2. Validate bot is Waiting and spot is Empty
3. Bot status: Waiting → Parked
4. Remove bot's PATIENCE event from queue
5. Add CHECKOUT event to queue

**When called:** Player assigns a waiting bot to an empty spot

---

### 3. checkout_bot

**What it does:**

1. **Process event queue** (includes auto-checkouts)
2. If manually called for a specific bot:
   - Validate bot is Parked and checkout time passed
   - Pay player $PARK from treasury
   - Update bot stats (total_visits++, total_paid+=)
   - Bot status: Parked → Idle
   - Free up the spot
   - Schedule next ARRIVAL event

**When called:** Player collects payment (or automatic via queue processing)

---

### 4. refresh

**What it does:**

1. **Process event queue** (that's it)

**When called:** Player opens app, client auto-sends to sync state

---

### 5. request_upgrade

**What it does:**

1. **Process event queue**
2. Validate player has enough $PARK
3. Transfer $PARK from player to treasury
4. Set upgrade_requested_at = now
5. Set upgrade_paid = true

**When called:** Player requests to upgrade to next level

---

### 6. complete_upgrade

**What it does:**

1. **Process event queue**
2. Validate wait time has passed (30 min or 45 min)
3. Find bot with highest total_paid → Promote to VIP
4. Create new bot accounts (2 for L2, 3 for L3)
5. Increase lot level
6. Increase spot count

**When called:** Player completes upgrade after wait time

---

## 🔑 The Key Insight

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  BLOCKCHAIN CAN'T RUN BACKGROUND PROCESSES.                             │
│                                                                         │
│  BUT...                                                                 │
│                                                                         │
│  We store WHAT SHOULD HAPPEN and WHEN.                                  │
│  When player interacts, we CALCULATE what DID happen.                   │
│  Then we show them the result.                                          │
│                                                                         │
│  To the player: Bots came, waited, left, paid — all while offline.      │
│  In reality: We just did the math when they came back.                  │
│                                                                         │
│  Same result. No server needed. Fully on-chain.                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 📊 Technical Feasibility

### Storage

| Data | Size | Cost (Light Protocol) |
|------|------|----------------------|
| Event (type + bot + time) | ~10 bytes | — |
| Max events (10 bots × 2) | ~200 bytes | — |
| Lot account | ~300 bytes | ~0.00005 SOL |
| Bot account | ~100 bytes | ~0.000017 SOL |
| 10 bots | ~1000 bytes | ~0.00017 SOL |
| **Total per player** | ~1.5 KB | **~$0.05** |

### Compute

| Operation | Compute Units |
|-----------|---------------|
| Process 1 event | ~600 CU |
| Process 10 events (worst case) | ~6,000 CU |
| Solana limit | 200,000 CU |
| **Verdict** | ✅ Plenty of room |

---

## 🗺️ What's NOT in MVP

- Facilities (Garage, Shop, etc.)
- Shopper/Repair bot types
- Multiplayer/Social features
- SOL bonds
- Token trading (DEX)

These come after MVP is working.

---

## Next Steps

1. [ ] Implement Lot account structure
2. [ ] Implement Bot account structure
3. [ ] Implement Event Queue processing logic
4. [ ] Build initialize_lot instruction
5. [ ] Build assign_bot instruction
6. [ ] Build checkout_bot instruction
7. [ ] Build upgrade flow (request + complete)
8. [ ] Test full Level 1 → 2 → 3 progression
9. [ ] Build frontend
