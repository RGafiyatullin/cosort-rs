use crate::CoSortTable;

#[test]
fn ergonomics_array() {
    let mut primary = [3, 1, 4, 2, 0];
    let mut secondary_a = ["three", "one", "four", "two", "zero"];
    let mut secondary_b = ["3", "1", "4", "2", "0"];

    eprintln!("{:#?}", primary);
    eprintln!("{:#?}", secondary_a);
    eprintln!("{:#?}", secondary_b);

    eprintln!("====");

    primary
        .add_column(&mut secondary_a)
        .add_column(&mut secondary_b)
        .cosort_unstable(&mut [Default::default(); 5])
        .expect("cosort-unstable failed");

    eprintln!("====");

    eprintln!("{:#?}", primary);
    eprintln!("{:#?}", secondary_a);
    eprintln!("{:#?}", secondary_b);
}
