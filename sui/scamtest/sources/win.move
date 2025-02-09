module scamtest::win;

// === Imports ===

use sui::coin;
use sui::url;
use scamtest::scamtest;
use scamtest::tst::TST;

// === Errors ===

// === Constants ===

// === Structs ===

public struct WIN has drop {}

fun init(witness: WIN, ctx: &mut TxContext) {
    let (output_treasury_cap, output_metadata) = coin::create_currency<WIN>(
        witness,
        9,
        b"WIN",
        b"You win",
        b"The winner takes it all",
        option::some(url::new_unsafe_from_bytes(
            b"https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSwK7FrW58c-a1Xg3AaWwHxn3YPVWWLjB8LukavFqY1xJOB_O8ApPwzNNri1-hnQ6axVAo&usqp=CAU"
        )),
        ctx
    );
    transfer::public_freeze_object(output_metadata);
    let (scamtest, admin_cap) = scamtest::new<TST, WIN>(
        output_treasury_cap.treasury_into_supply(),
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
    init(WIN {}, ctx)
}
