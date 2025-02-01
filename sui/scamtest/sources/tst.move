module scamtest::tst;

// === Imports ===

use sui::coin;
use sui::url;
use scamtest::scamtest;

// === Errors ===

// === Constants ===

// === Structs ===

public struct TST has drop {}

fun init(witness: TST, ctx: &mut TxContext) {
    let (treasury_cap, metadata) = coin::create_currency<TST>(
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
    transfer::public_freeze_object(metadata);
    let (scamtest, admin_cap) = scamtest::new(
        treasury_cap.treasury_into_supply(),
        ctx
    );
    scamtest.share();
    transfer::public_transfer(
        admin_cap,
        ctx.sender()
    );
}

// === Events ===

// === Method Aliases ===

// === Entry Functions ===

// === Public Functions ===

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