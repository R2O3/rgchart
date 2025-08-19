mod test_stuff;
use test_stuff::*;

#[test]
fn osu_to_qua_test() {
    parse_and_convert!(
        osu_to_qua,
        "./tests/Maps/osu/165991_PlusDanshi/Reol - +Danshi (lZenxl) [7K OVERLOAD].osu",
        parse::from_osu,
        write::to_qua,
        true
    );
}

#[test]
fn sm_to_qua_test() {
    parse_and_convert!(
        sm_to_qua,
        "./tests/Maps/etterna/Kil_ChineseTea/ct.sm",
        parse::from_sm,
        write::to_qua,
        true
    );
}

#[test]
fn fsc_to_qua_test() {
    parse_and_convert!(
        fsc_to_qua,
        "./tests/Maps/fluXis/225_BimboLimbo/1720743020.fsc",
        parse::from_fsc,
        write::to_qua,
        true
    );
}