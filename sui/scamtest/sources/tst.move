module scamtest::tst;

// === Imports ===

use sui::coin::{Self, TreasuryCap, Coin};
use sui::balance::Balance;
use sui::url;
use sui::event;

// === Errors ===

// === Constants ===

const MINT_AMOUNT: u64 = 10_000_000_000;

// === Structs ===

public struct TST has drop {}

fun init(witness: TST, ctx: &mut TxContext) {
    let (input_treasury_cap, input_metadata) = coin::create_currency<TST>(
        witness,
        9,
        b"TST",
        b"Test coin",
        b"Coin made for the testing purposes",
        option::some(url::new_unsafe_from_bytes(
            b"https://tvavodwlujeccwmu5duaceam7cvk4cnesrvojvkhsk6lytokheda.arweave.net/nUFXDsuiSCFZlOjoARAM-KquCaSUauTVR5K8vE3KOQY"
        )),
        ctx
    );
    transfer::public_freeze_object(input_metadata);
    transfer::public_share_object(
        input_treasury_cap,
    );
}

// === Events ===

public struct MintedTstEvent has copy, drop {
    amount: u64,
}

public struct BurnedTstEvent has copy, drop {
    amount: u64,
}

// === Method Aliases ===

// === Entry Functions ===

public entry fun mint_to(treasury_cap: &mut TreasuryCap<TST>, ctx: &mut TxContext) {
    let coin = treasury_cap.mint(MINT_AMOUNT, ctx);
    transfer::public_transfer(
        coin,
        ctx.sender()
    );
    event::emit(MintedTstEvent { amount: MINT_AMOUNT });
}

public entry fun burn(treasury_cap: &mut TreasuryCap<TST>, coin: Coin<TST>) {
    event::emit(BurnedTstEvent { amount: coin.value() });
    treasury_cap.burn(coin);
}

// === Public Functions ===

public fun mint_balance(treasury_cap: &mut TreasuryCap<TST>): Balance<TST> {
    event::emit(MintedTstEvent { amount: MINT_AMOUNT });
    treasury_cap.supply_mut().increase_supply(MINT_AMOUNT)
}

public fun burn_balance(treasury_cap: &mut TreasuryCap<TST>, balance: Balance<TST>): u64 {
    event::emit(BurnedTstEvent { amount: balance.value() });
    treasury_cap.supply_mut().decrease_supply(balance)
}

// === View Functions ===

// === Admin Functions ===

// === Package Functions ===

// === Private Functions ===

// === Test Functions ===

#[test_only]
/// Wrapper of module initializer for testing
public fun test_init(ctx: &mut TxContext) {
    init(TST {}, ctx)
}