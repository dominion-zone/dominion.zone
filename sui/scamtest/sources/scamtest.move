module scamtest::scamtest;

// === Imports ===

use sui::balance::{Self, Balance, Supply};
use sui::coin::{Self, Coin};
use sui::vec_set::{Self, VecSet};
use sui::vec_map::{Self, VecMap};
use sui::hash;
use sui::event;
use std::type_name::{Self, TypeName};
use sui::clock::Clock;

// === Errors ===

#[error]
const InvalidAdminCap: vector<u8> = b"The object id of the admin cap is unexpected";

#[error]
const InvalidOperatorCap: vector<u8> = b"Unexpected operator cap";

#[error]
const BlacklistedOperator: vector<u8> = b"Operator is blacklisted";

// === Constants ===

// === Structs ===

public struct Scamtest<phantom I, phantom O> has key {
    id: UID,
    admin_cap_id: ID,
    supply: Supply<O>,
    scammed: Balance<I>,
    slots: VecMap<vector<u8>, u64>,
    operator_balcklist: VecSet<ID>,
}

public struct AdminCap<phantom I, phantom O> has key, store {
    id: UID,
    scamtest_id: ID,
}

public struct OperatorCap<phantom I, phantom O> has key, store {
    id: UID,
    scamtest_id: ID,
}

// === Events ===

public struct CreatedEvent has copy, drop {
    scamtest_id: ID,
    admin_cap_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
}

public struct SharedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
}

public struct OperatorCreatedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    operator_id: ID,
}

public struct OperatorDestroyedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    operator_id: ID,
}

public struct OperatorBlacklistedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    operator_id: ID,
}

public struct SlotAddedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    operator_id: ID,
    slot: vector<u8>,
    expiration: u64,
}

public struct SlotRemovedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    operator_id: Option<ID>,
    slot: vector<u8>,
    expiration: u64,
    time: u64,
}

public struct WinEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    amount: u64,
    secret: vector<u8>,
    slot: vector<u8>,
}

public struct LoseEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    amount: u64,
}

public struct BurnedEvent has copy, drop {
    scamtest_id: ID,
    input_coin_type: TypeName,
    output_coin_type: TypeName,
    amount: u64,
}

// === Method Aliases ===

public use fun admin_cap_scamtest_id as AdminCap.scamtest_id;
public use fun operator_cap_scamtest_id as OperatorCap.scamtest_id;

// === Entry functions ===

entry fun place_bet_to<I, O>(
    self: &mut Scamtest<I, O>,
    bet: Coin<I>,
    secret: vector<u8>,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let amount = bet.value();
    self.scammed.join(bet.into_balance());
    if (self.check_secret(amount, secret, clock)) {
        let win = self.supply.increase_supply(amount);
        transfer::public_transfer(
            coin::from_balance(win, ctx), 
            ctx.sender()
        );
    }
}

entry fun burn<I, O>(
    self: &mut Scamtest<I, O>,
    coin: Coin<O>,
) {
    self.burn_balance(coin.into_balance());
}

public entry fun cleanup_slots<I, O>(
    self: &mut Scamtest<I, O>,
    clock: &Clock,
) {
    let mut i = self.slots.size() - 1;
    loop {
        let (slot, expiration) = self.slots.get_entry_by_idx(i);
        let slot = *slot;
        let expiration = *expiration;
        if (clock.timestamp_ms() >= expiration) {
            self.slots.remove_entry_by_idx(i);
            event::emit(SlotRemovedEvent {
                scamtest_id: object::id(self),
                input_coin_type: type_name::get_with_original_ids<I>(),
                output_coin_type: type_name::get_with_original_ids<O>(),
                operator_id: option::none(),
                slot,
                expiration,
                time: clock.timestamp_ms(),
            });
        };
        if (i == 0) {
            break
        };
        i = i - 1;
    }
}

entry fun new_operator_owned<I, O>(
    self: &AdminCap<I, O>,
    scamtest: &Scamtest<I, O>,
    ctx: &mut TxContext,
) {
    let operator = self.new_operator(scamtest, ctx);
    transfer::transfer(operator, ctx.sender());
}

public entry fun blacklist_operator<I, O>(
    self: &AdminCap<I, O>,
    scamtest: &mut Scamtest<I, O>,
    operator_id: ID,
) {
    self.assert_admin_cap(scamtest);
    scamtest.operator_balcklist.insert(operator_id);
    event::emit(OperatorBlacklistedEvent {
        scamtest_id: object::id(scamtest),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        operator_id,
    });
}


public entry fun destroy_operator<I, O>(
    self: OperatorCap<I, O>,
    scamtest: &mut Scamtest<I, O>,
) {
    let OperatorCap {
        id,
        ..
    } = self;
    if (scamtest.operator_balcklist.contains(id.as_inner())) {
        scamtest.operator_balcklist.remove(id.as_inner());
    };
    event::emit(OperatorDestroyedEvent {
        scamtest_id: object::id(scamtest),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        operator_id: id.to_inner(),
    });
    id.delete();
}

public entry fun add_slot<I, O>(
    self: &OperatorCap<I, O>,
    scamtest: &mut Scamtest<I, O>,
    slot: vector<u8>,
    timeout: u64,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let expiration = clock.timestamp_ms() + timeout;
    scamtest.slots.insert(slot, expiration);
    event::emit(SlotAddedEvent {
        scamtest_id: object::id(scamtest),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        operator_id: object::id(self),
        slot,
        expiration,
    });
}

public entry fun remove_slot<I, O>(
    self: &OperatorCap<I, O>,
    scamtest: &mut Scamtest<I, O>,
    slot: vector<u8>,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let (_, expiration) = scamtest.slots.remove(&slot);
    event::emit(SlotRemovedEvent {
        scamtest_id: object::id(scamtest),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        operator_id: option::some(object::id(self)),
        slot,
        expiration,
        time: clock.timestamp_ms(),
    });
}

public entry fun reset_slots<I, O>(
    self: &OperatorCap<I, O>,
    scamtest: &mut Scamtest<I, O>,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let mut i = scamtest.slots.size() - 1;
    loop {
        let (slot, expiration) = scamtest.slots.remove_entry_by_idx(i);
        event::emit(SlotRemovedEvent {
            scamtest_id: object::id(scamtest),
            input_coin_type: type_name::get_with_original_ids<I>(),
            output_coin_type: type_name::get_with_original_ids<O>(),
            operator_id: option::some(object::id(self)),
            slot,
            expiration,
            time: clock.timestamp_ms(),
        });
        if (i == 0) {
            break
        };
        i = i - 1;
    }
}

// === Public Functions ===

public fun place_bet_balance<I, O>(
    self: &mut Scamtest<I, O>,
    bet: Balance<I>,
    secret: vector<u8>,
    clock: &Clock,
): Balance<O> {
    let amount = bet.value();
    self.scammed.join(bet);
    if (self.check_secret(amount, secret, clock)) {
        self.supply.increase_supply(amount)
    } else {
        balance::zero()
    }
}

public fun burn_balance<I, O>(
    self: &mut Scamtest<I, O>,
    balance: Balance<O>,
) {
    event::emit(BurnedEvent {
        scamtest_id: object::id(self),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        amount: balance.value(),
    });
    self.supply.decrease_supply(balance);
}

// === View Functions ===

public fun admin_cap_id<I, O>(
    self: &Scamtest<I, O>,
): ID {
    self.admin_cap_id
}

public fun supply_value<I, O>(
    self: &Scamtest<I, O>,
): u64 {
    self.supply.supply_value()
}

public fun scammed_value<I, O>(
    self: &Scamtest<I, O>,
): u64 {
    self.scammed.value()
}

public fun admin_cap_scamtest_id<I, O>(
    self: &AdminCap<I, O>,
): ID {
    self.scamtest_id
}

public fun operator_cap_scamtest_id<I, O>(
    self: &OperatorCap<I, O>,
): ID {
    self.scamtest_id
}

// === Admin Functions ===

public fun new<I, O>(
    supply: Supply<O>,
    ctx: &mut TxContext,
): (Scamtest<I, O>, AdminCap<I, O>) {
    let scamtest_uid = object::new(ctx);
    let scamtest_id = scamtest_uid.to_inner();
    let admin_cap_uid = object::new(ctx);
    let admin_cap_id = admin_cap_uid.to_inner();
    let scamtest = Scamtest<I, O> {
        id: scamtest_uid,
        admin_cap_id,
        supply,
        scammed: balance::zero(),
        slots: vec_map::empty(),
        operator_balcklist: vec_set::empty(),
    };

    let admin_cap = AdminCap<I, O> {
        id: admin_cap_uid,
        scamtest_id,
    };

    event::emit(CreatedEvent {
        scamtest_id,
        admin_cap_id,
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
    });

    (scamtest, admin_cap)
}

public fun share<I, O>(
    self: Scamtest<I, O>,
) {
    event::emit(SharedEvent {
        scamtest_id: object::id(&self),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
    });
    transfer::share_object(self)
}

public fun assert_admin_cap<I, O>(
    self: &AdminCap<I, O>,
    scamtest: &Scamtest<I, O>,
) {
    assert!(scamtest.admin_cap_id == object::id(self), InvalidAdminCap);
}

public fun new_operator<I, O>(
    self: &AdminCap<I, O>,
    scamtest: &Scamtest<I, O>,
    ctx: &mut TxContext,
): OperatorCap<I, O> {
    self.assert_admin_cap(scamtest);
    let operator_uid = object::new(ctx);
    event::emit(OperatorCreatedEvent {
        scamtest_id: object::id(scamtest),
        input_coin_type: type_name::get_with_original_ids<I>(),
        output_coin_type: type_name::get_with_original_ids<O>(),
        operator_id: operator_uid.to_inner(),
    });
    OperatorCap<I, O> {
        id: operator_uid,
        scamtest_id: object::id(scamtest),
    }
}

// === Operator Functions ===

public fun assert_operator_cap<I, O>(
    self: &OperatorCap<I, O>,
    scamtest: &Scamtest<I, O>,
) {
    assert!(self.scamtest_id == object::id(scamtest), InvalidOperatorCap);
    assert!(
        !scamtest.operator_balcklist.contains(object::borrow_id(self)),
        BlacklistedOperator,
    );
}

// === Package Functions ===

// === Private Functions ===

public fun check_secret<I, O>(
    self: &mut Scamtest<I, O>,
    amount: u64,
    secret: vector<u8>,
    clock: &Clock,
): bool {
    self.cleanup_slots(clock);
    let input_coin_type = type_name::get_with_original_ids<I>();
    let output_coin_type = type_name::get_with_original_ids<O>();
    let slot = hash::keccak256(&secret);
    if (self.slots.contains(&slot)) {
        self.slots.remove(&slot);
        event::emit(WinEvent {
            scamtest_id: object::id(self),
            input_coin_type,
            output_coin_type,
            amount,
            secret,
            slot,
        });
        true
    } else {
        event::emit(LoseEvent {
            scamtest_id: object::id(self),
            input_coin_type,
            output_coin_type,
            amount,
        });
        false
    }
}

// === Test Functions ===

/*
{
    "supply_value": {
        "name": "supply_value",
        "params": [
            {
                "name": "scamtest",
                "type": "&Scamtest<T0>",
                "description": "Reference to the Scamtest instance to query."
            }
        ],
        "side_effects": [],
        "description": "Returns the total supply value of the token managed by the Scamtest contract.",
        "score": "safe",
        "warnings": [],
        "returns": [
            {
                "name": "supply",
                "type": "u64",
                "description": "Total supply of the token."
            }
        ]
    },
    "new": {
        "name": "new",
        "params": [
            {
                "name": "supply",
                "type": "0x2::balance::Supply<T0>",
                "description": "Initial token supply to manage."
            },
            {
                "name": "tx_context",
                "type": "&mut 0x2::tx_context::TxContext",
                "description": "Transaction context for object creation."
            }
        ],
        "side_effects": [
            {
                "effect": "0x2::transfer::public_transfer",
                "details": "Transfers newly created AdminCap to the caller. Object is newly created within the function."
            }
        ],
        "description": "Initializes a new Scamtest instance with initial supply and creates associated AdminCap. Emits CreatedEvent.",
        "score": "safe",
        "warnings": [],
        "returns": [
            {
                "name": "scamtest_instance",
                "type": "Scamtest<T0>",
                "description": "Newly created Scamtest instance."
            },
            {
                "name": "admin_cap",
                "type": "AdminCap<T0>",
                "description": "Admin capability object for managing the instance."
            }
        ]
    },
    "add_slot": {
        "name": "add_slot",
        "params": [
            {
                "name": "operator_cap",
                "type": "&OperatorCap<T0>",
                "description": "Authorization capability for slot operations."
            },
            {
                "name": "scamtest",
                "type": "&mut Scamtest<T0>",
                "description": "Mutable reference to Scamtest instance."
            },
            {
                "name": "slot_key",
                "type": "vector<u8>",
                "description": "Byte vector identifying the slot."
            },
            {
                "name": "validity_period",
                "type": "u64",
                "description": "Duration in milliseconds how long the slot remains valid."
            },
            {
                "name": "clock",
                "type": "&0x2::clock::Clock",
                "description": "Clock reference for timestamp validation."
            }
        ],
        "side_effects": [
            {
                "effect": "(Logging) emits SlotAddedEvent",
                "details": "Records addition of new slot with expiration timestamp."
            }
        ],
        "description": "Adds a new slot entry with expiration timestamp. Requires valid OperatorCap and not blacklisted operator.",
        "score": "risky",
        "warnings": [
            "Slot validity period relies on client-side clock which could be manipulated in test environments.",
            "Operator permissions not validated against admin cap - potential privilege escalation risk."
        ],
        "returns": []
    },
    "blacklist_operator": {
        "name": "blacklist_operator",
        "params": [
            {
                "name": "admin_cap",
                "type": "&AdminCap<T0>",
                "description": "Admin capability proving authorization."
            },
            {
                "name": "scamtest",
                "type": "&mut Scamtest<T0>",
                "description": "Mutable Scamtest instance to modify."
            },
            {
                "name": "operator_id",
                "type": "0x2::object::ID",
                "description": "ID of operator to blacklist."
            }
        ],
        "side_effects": [
            {
                "effect": "(Logging) emits OperatorBlacklistedEvent",
                "details": "Records operator blacklisting."
            }
        ],
        "description": "Adds operator ID to blacklist preventing future operations. Requires AdminCap authorization.",
        "score": "safe",
        "warnings": [
            "Irreversible operation with no un-blacklist functionality visible."
        ],
        "returns": []
    },
    "burn": {
        "name": "burn",
        "params": [
            {
                "name": "scamtest",
                "type": "&mut Scamtest<T0>",
                "description": "Mutable reference to Scamtest instance."
            },
            {
                "name": "coin",
                "type": "0x2::coin::Coin<T0>",
                "description": "Coin to burn."
            }
        ],
        "side_effects": [
            {
                "effect": "(Logging) emits BurnedEvent",
                "details": "Records burn operation details."
            },
            {
                "effect": "Reduces token supply",
                "details": "Permanent reduction of total supply via 0x2::balance::decrease_supply."
            }
        ],
        "description": "Burns specified amount of tokens permanently. Decreases total supply.",
        "score": "high-risk",
        "warnings": [
            "Permanent token destruction mechanism - could lead to accidental asset loss.",
            "No authorization check - possibly should require admin cap."
        ],
        "returns": []
    },
    "mint_to": {
        "name": "mint_to",
        "params": [
            {
                "name": "scamtest",
                "type": "&mut Scamtest<T0>",
                "description": "Mutable Scamtest instance."
            },
            {
                "name": "tx_context",
                "type": "&mut 0x2::tx_context::TxContext",
                "description": "Tx context for sender address."
            }
        ],
        "side_effects": [
            {
                "effect": "0x2::transfer::public_transfer of minted coins",
                "details": "Mints fixed amount (10^10 units) and transfers to sender. New coins created and transferred."
            }
        ],
        "description": "Mints fixed amount of tokens and sends them to transaction sender. No cap validation visible.",
        "score": "critical-risk",
        "warnings": [
            "Uncontrolled minting without proper authorization checks.",
            "Fixed large mint amount (10^10) could lead to inflation vulnerabilities."
        ],
        "returns": []
    },
    "new_operator_owned": {
        "name": "new_operator_owned",
        "params": [
            {
                "name": "admin_cap",
                "type": "&AdminCap<T0>",
                "description": "Admin capability to authorize creation."
            },
            {
                "name": "scamtest",
                "type": "&Scamtest<T0>",
                "description": "Scamtest instance reference."
            },
            {
                "name": "tx_context",
                "type": "&mut 0x2::tx_context::TxContext",
                "description": "Tx context for object creation."
            }
        ],
        "side_effects": [
            {
                "effect": "0x2::transfer::transfer of new OperatorCap",
                "details": "Transfers newly created OperatorCap to transaction sender address."
            }
        ],
        "description": "Creates and transfers new Operator capability to sender. Requires AdminCap authorization.",
        "score": "safe",
        "warnings": [],
        "returns": []
    },
    "check_secret": {
        "name": "check_secret",
        "params": [
            {
                "name": "scamtest",
                "type": "&mut Scamtest<T0>",
                "description": "Mutable Scamtest instance."
            },
            {
                "name": "bet_amount",
                "type": "u64",
                "description": "Amount wagered in the bet."
            },
            {
                "name": "secret",
                "type": "vector<u8>",
                "description": "User-provided secret for slot check."
            },
            {
                "name": "clock",
                "type": "&0x2::clock::Clock",
                "description": "Clock for timestamp validation."
            }
        ],
        "side_effects": [
            {
                "effect": "Conditional token minting/confiscation",
                "details": "If secret hash matches existing slot: mints equivalent amount to user. Else, confiscates bet to 'scammed' balance. Decision depends on slot validity and hash match."
            },
            {
                "effect": "Emits WinEvent or LoseEvent",
                "details": "Logs outcome of bet attempt."
            }
        ],
        "description": "Core gambling mechanic - checks secret against active slots. Mints reward or confiscates bet based on result.",
        "score": "critical-risk",
        "warnings": [
            "High-risk gambling logic with direct funds control.",
            "Slot verification uses simple hash check - vulnerable to brute-force if slot generation isn't secure.",
            "Confiscated funds go to inaccessible 'scammed' balance with unclear recovery mechanism."
        ],
        "returns": [
            {
                "name": "is_winner",
                "type": "bool",
                "description": "True if secret matched valid slot, false otherwise."
            }
        ]
    }

}
*/