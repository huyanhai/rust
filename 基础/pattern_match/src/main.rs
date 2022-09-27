mod deconstruct;
mod matchs;
mod options;

fn main() {
    matchs::test_matchs();

    options::test_option();

    deconstruct::test_dec();
}
