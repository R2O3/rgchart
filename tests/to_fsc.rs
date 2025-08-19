mod test_stuff;
use test_stuff::*;

#[test]
fn sm_to_fsc_test() {
    parse_and_convert!(
        sm_to_fsc,
        "./tests/Maps/etterna/Kil_ChineseTea/ct.sm",
        parse::from_sm,
        write::to_fsc,
        true
    );
}

#[test]
fn qua_to_fsc_test() {
    parse_and_convert!(
        qua_to_fsc,
        "./tests/Maps/quaver/2366_177_NewNonBiyori/19248.qua",
        parse::from_qua,
        write::to_fsc,
        true
    );
}

#[test]
fn osu_to_fsc_test() {
    parse_and_convert!(
        osu_to_fsc,
        "./tests/Maps/osu/165991_PlusDanshi/Reol - +Danshi (lZenxl) [7K OVERLOAD].osu",
        parse::from_osu,
        write::to_fsc,
        true
    );
}
