mod advanced;
mod gen;
mod traits;
mod traits_obj;

fn main() {
    gen::test_gen();

    traits::test_traits();

    traits_obj::test_obj();

    advanced::test_adv();
}
