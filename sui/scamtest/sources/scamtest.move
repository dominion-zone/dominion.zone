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

const MINT_AMOUNT: u64 = 10_000_000_000;

// === Structs ===

public struct Scamtest<phantom C> has key {
    id: UID,
    admin_cap_id: ID,
    supply: Supply<C>,
    scammed: Balance<C>,
    slots: VecMap<vector<u8>, u64>,
    operator_balcklist: VecSet<ID>,
}

public struct AdminCap<phantom C> has key, store {
    id: UID,
    scamtest_id: ID,
}

public struct OperatorCap<phantom C> has key, store {
    id: UID,
    scamtest_id: ID,
}

// === Events ===

public struct CreatedEvent has copy, drop {
    scamtest_id: ID,
    admin_cap_id: ID,
    coin_type: TypeName,
}

public struct SharedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
}

public struct OperatorCreatedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    operator_id: ID,
}

public struct OperatorDestroyedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    operator_id: ID,
}

public struct OperatorBlacklistedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    operator_id: ID,
}

public struct SlotAddedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    operator_id: ID,
    slot: vector<u8>,
    expiration: u64,
}

public struct SlotRemovedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    operator_id: Option<ID>,
    slot: vector<u8>,
    expiration: u64,
    time: u64,
}

public struct WinEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    amount: u64,
    secret: vector<u8>,
    slot: vector<u8>,
}

public struct LoseEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    amount: u64,
}

public struct MintedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
}

public struct BurnedEvent has copy, drop {
    scamtest_id: ID,
    coin_type: TypeName,
    amount: u64,
}

// === Method Aliases ===

public use fun admin_cap_scamtest_id as AdminCap.scamtest_id;
public use fun operator_cap_scamtest_id as OperatorCap.scamtest_id;

// === Entry functions ===

entry fun place_bet_mut<C>(
    self: &mut Scamtest<C>,
    bet: &mut Coin<C>,
    secret: vector<u8>,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let amount = bet.value();
    if (self.check_secret(amount, secret, clock)) {
        bet.join(self.supply.increase_supply(amount).into_coin(ctx));
    } else {
        self.scammed.join(bet.balance_mut().withdraw_all());
    }
}

entry fun place_bet_to<C>(
    self: &mut Scamtest<C>,
    bet: Coin<C>,
    secret: vector<u8>,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let amount = bet.value();
    if (self.check_secret(amount, secret, clock)) {
        let mut win = self.supply.increase_supply(amount);
        win.join(bet.into_balance());
        transfer::public_transfer(
            coin::from_balance(win, ctx), 
            ctx.sender()
        );
    } else {
        self.scammed.join(bet.into_balance());
    };
}

entry fun mint_to<C>(
    self: &mut Scamtest<C>,
    ctx: &mut TxContext,
) {
    let balance = self.mint_balance();
    transfer::public_transfer(
        coin::from_balance(balance, ctx),
        ctx.sender()
    );
}

entry fun burn<C>(
    self: &mut Scamtest<C>,
    coin: Coin<C>,
) {
    self.burn_balance(coin.into_balance());
}

public entry fun cleanup_slots<C>(
    self: &mut Scamtest<C>,
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
                coin_type: type_name::get_with_original_ids<C>(),
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

entry fun new_operator_owned<C>(
    self: &AdminCap<C>,
    scamtest: &Scamtest<C>,
    ctx: &mut TxContext,
) {
    let operator = self.new_operator(scamtest, ctx);
    transfer::transfer(operator, ctx.sender());
}

public entry fun blacklist_operator<C>(
    self: &AdminCap<C>,
    scamtest: &mut Scamtest<C>,
    operator_id: ID,
) {
    self.assert_admin_cap(scamtest);
    scamtest.operator_balcklist.insert(operator_id);
    event::emit(OperatorBlacklistedEvent {
        scamtest_id: object::id(scamtest),
        coin_type: type_name::get_with_original_ids<C>(),
        operator_id,
    });
}


public entry fun destroy_operator<C>(
    self: OperatorCap<C>,
    scamtest: &mut Scamtest<C>,
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
        coin_type: type_name::get_with_original_ids<C>(),
        operator_id: id.to_inner(),
    });
    id.delete();
}

public entry fun add_slot<C>(
    self: &OperatorCap<C>,
    scamtest: &mut Scamtest<C>,
    slot: vector<u8>,
    timeout: u64,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let expiration = clock.timestamp_ms() + timeout;
    scamtest.slots.insert(slot, expiration);
    event::emit(SlotAddedEvent {
        scamtest_id: object::id(scamtest),
        coin_type: type_name::get_with_original_ids<C>(),
        operator_id: object::id(self),
        slot,
        expiration,
    });
}

public entry fun remove_slot<C>(
    self: &OperatorCap<C>,
    scamtest: &mut Scamtest<C>,
    slot: vector<u8>,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let (_, expiration) = scamtest.slots.remove(&slot);
    event::emit(SlotRemovedEvent {
        scamtest_id: object::id(scamtest),
        coin_type: type_name::get_with_original_ids<C>(),
        operator_id: option::some(object::id(self)),
        slot,
        expiration,
        time: clock.timestamp_ms(),
    });
}

public entry fun reset_slots<C>(
    self: &OperatorCap<C>,
    scamtest: &mut Scamtest<C>,
    clock: &Clock,
) {
    self.assert_operator_cap(scamtest);
    let mut i = scamtest.slots.size() - 1;
    loop {
        let (slot, expiration) = scamtest.slots.remove_entry_by_idx(i);
        event::emit(SlotRemovedEvent {
            scamtest_id: object::id(scamtest),
            coin_type: type_name::get_with_original_ids<C>(),
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

public fun place_bet_balance_mut<C>(
    self: &mut Scamtest<C>,
    bet: &mut Balance<C>,
    secret: vector<u8>,
    clock: &Clock,
) {
    let amount = bet.value();
    if (self.check_secret(amount, secret, clock)) {
        bet.join(self.supply.increase_supply(amount));
    } else {
        self.scammed.join(bet.withdraw_all());
    }
}

public fun place_bet_balance<C>(
    self: &mut Scamtest<C>,
    bet: Balance<C>,
    secret: vector<u8>,
    clock: &Clock,
): Balance<C> {
    let amount = bet.value();
    if (self.check_secret(amount, secret, clock)) {
        let mut win = self.supply.increase_supply(amount);
        win.join(bet);
        win
    } else {
        self.scammed.join(bet);
        balance::zero()
    }
}

public fun mint_balance<C>(
    self: &mut Scamtest<C>,
): Balance<C> {
    event::emit(MintedEvent {
        scamtest_id: object::id(self),
        coin_type: type_name::get_with_original_ids<C>(),
    });
    self.supply.increase_supply(MINT_AMOUNT)
}

public fun burn_balance<C>(
    self: &mut Scamtest<C>,
    balance: Balance<C>,
) {
    event::emit(BurnedEvent {
        scamtest_id: object::id(self),
        coin_type: type_name::get_with_original_ids<C>(),
        amount: balance.value(),
    });
    self.supply.decrease_supply(balance);
}

// === View Functions ===

public fun admin_cap_id<C>(
    self: &Scamtest<C>,
): ID {
    self.admin_cap_id
}

public fun supply_value<C>(
    self: &Scamtest<C>,
): u64 {
    self.supply.supply_value()
}

public fun scammed_value<C>(
    self: &Scamtest<C>,
): u64 {
    self.scammed.value()
}

public fun admin_cap_scamtest_id<C>(
    self: &AdminCap<C>,
): ID {
    self.scamtest_id
}

public fun operator_cap_scamtest_id<C>(
    self: &OperatorCap<C>,
): ID {
    self.scamtest_id
}

// === Admin Functions ===

public fun new<C>(
    supply: Supply<C>,
    ctx: &mut TxContext,
): (Scamtest<C>, AdminCap<C>) {
    let scamtest_uid = object::new(ctx);
    let scamtest_id = scamtest_uid.to_inner();
    let admin_cap_uid = object::new(ctx);
    let admin_cap_id = admin_cap_uid.to_inner();
    let scamtest = Scamtest<C> {
        id: scamtest_uid,
        admin_cap_id,
        supply,
        scammed: balance::zero(),
        slots: vec_map::empty(),
        operator_balcklist: vec_set::empty(),
    };

    let admin_cap = AdminCap<C> {
        id: admin_cap_uid,
        scamtest_id,
    };

    event::emit(CreatedEvent {
        scamtest_id,
        admin_cap_id,
        coin_type: type_name::get_with_original_ids<C>(),
    });

    (scamtest, admin_cap)
}

public fun share<C>(
    self: Scamtest<C>,
) {
    event::emit(SharedEvent {
        scamtest_id: object::id(&self),
        coin_type: type_name::get_with_original_ids<C>(),
    });
    transfer::share_object(self)
}

public fun assert_admin_cap<C>(
    self: &AdminCap<C>,
    scamtest: &Scamtest<C>,
) {
    assert!(scamtest.admin_cap_id == object::id(self), InvalidAdminCap);
}

public fun new_operator<C>(
    self: &AdminCap<C>,
    scamtest: &Scamtest<C>,
    ctx: &mut TxContext,
): OperatorCap<C> {
    self.assert_admin_cap(scamtest);
    let operator_uid = object::new(ctx);
    event::emit(OperatorCreatedEvent {
        scamtest_id: object::id(scamtest),
        coin_type: type_name::get_with_original_ids<C>(),
        operator_id: operator_uid.to_inner(),
    });
    OperatorCap<C> {
        id: operator_uid,
        scamtest_id: object::id(scamtest),
    }
}

// === Operator Functions ===

public fun assert_operator_cap<C>(
    self: &OperatorCap<C>,
    scamtest: &Scamtest<C>,
) {
    assert!(self.scamtest_id == object::id(scamtest), InvalidOperatorCap);
    assert!(
        !scamtest.operator_balcklist.contains(object::borrow_id(self)),
        BlacklistedOperator,
    );
}

// === Package Functions ===

// === Private Functions ===

public fun check_secret<C>(
    self: &mut Scamtest<C>,
    amount: u64,
    secret: vector<u8>,
    clock: &Clock,
): bool {
    self.cleanup_slots(clock);
    let coin_type = type_name::get_with_original_ids<C>();
    let slot = hash::keccak256(&secret);
    if (self.slots.contains(&slot)) {
        self.slots.remove(&slot);
        event::emit(WinEvent {
            scamtest_id: object::id(self),
            coin_type,
            amount,
            secret,
            slot,
        });
        true
    } else {
        event::emit(LoseEvent {
            scamtest_id: object::id(self),
            coin_type,
            amount,
        });
        false
    }
}

// === Test Functions ===
