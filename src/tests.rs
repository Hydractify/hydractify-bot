#[test]
/// We test this since we convert from i64 (to use with sqlx) to u64 for
/// `poise::serenity_prelude::MessageId` instances.
///
/// Maybe stupid to do so, but I am afraid this will cause a problem someday.
fn i64_to_u64_conversion() {
    let signed: i64 = i64::MAX;

    let unsigned: u64 = u64::MAX / 2;

    // This means that as long as we're in the range of u64 with our i64 (which it always will)
    // we're safe to cast it to u64, as it uses two's complement for number representation.
    assert_eq!(signed as u64, unsigned);
}
