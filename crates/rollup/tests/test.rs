use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::utils::generate_address as gen_addr_generic;
use sov_modules_api::Address;

fn generate_address(name: &str) -> Address {
    gen_addr_generic::<DefaultContext>(name)
}

#[test]
fn test_generate_address() {
    // Preparation
    let admin = generate_address("admin");
    let owner1 = generate_address("owner2");
    let owner2 = generate_address("owner2");
    // let config: NonFungibleTokenConfig<C> = NonFungibleTokenConfig {
    //     admin,
    //     owners: vec![(0, owner1)],
    // };
    println!("{}", admin);
    println!("{}", owner1);
    println!("{}", owner2);
}
