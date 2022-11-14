#[cfg(test)]
mod gumball_tests
{
    use scrypto::dec;
    use scrypto::prelude::{Decimal};
    use suft::blueprint::Blueprint;
    use suft::method::{Arg, Method};
    use suft::method_args;
    use suft::test_environment::TestEnvironment;
    use suft::package::Package;

    struct GumballBp {}

    impl Blueprint for GumballBp
    {
        fn instantiate(&self) -> (&str, Vec<&str>) {
            let name = "instantiate_gumball_machine";
            let args = vec!["1.5"];

            (name, args)
        }

        fn name(&self) -> &str {
            "GumballMachine"
        }
    }

    enum GumballMethods
    {
        GetPrice,
        BuyGumball(Decimal)
    }

    impl Method for GumballMethods
    {
        fn name(&self) -> &str {
            match self
            {
                GumballMethods::GetPrice => { "get_price" }
                GumballMethods::BuyGumball(_) => { "buy_gumball" }
            }
        }

        fn args(&self) -> Option<Vec<Arg>> {
            match self
            {
                GumballMethods::GetPrice => { method_args![] }
                GumballMethods::BuyGumball(value) =>
                    {
                        method_args![Arg::BucketArg(String::from("radix"), value.clone())]
                    }
            }
        }
    }


    #[test]
    fn test_publish()
    {
        let mut test_env = TestEnvironment::new();
        let gumball_blueprint = Box::new(GumballBp{});
        let mut gumball_package = Package::new("tests/assets/gumball-machine/");
        gumball_package.add_blueprint("gumball", gumball_blueprint);
        test_env.publish_package("gumball", gumball_package);
    }

    #[test]
    fn test_instantiate()
    {
        let mut test_env = TestEnvironment::new();
        let gumball_blueprint = Box::new(GumballBp{});
        let mut gumball_package = Package::new("tests/assets/gumball-machine/");
        gumball_package.add_blueprint("gumball", gumball_blueprint);
        test_env.publish_package("gumball", gumball_package);
        test_env.new_component("gumball_comp", "gumball", "gumball");

        test_env.get_token("gumball").unwrap();
    }

    #[test]
    fn test_get_price()
    {
        let mut test_env = TestEnvironment::new();
        let gumball_blueprint = Box::new(GumballBp{});
        let mut gumball_package = Package::new("tests/assets/gumball-machine/");
        gumball_package.add_blueprint("gumball", gumball_blueprint);
        test_env.publish_package("gumball", gumball_package);
        test_env.new_component("gumball_comp", "gumball", "gumball");

        test_env.call_method("gumball_comp", GumballMethods::GetPrice);
    }

    #[test]
    fn test_buy_gumball()
    {
        let mut test_env = TestEnvironment::new();
        let gumball_blueprint = Box::new(GumballBp{});
        let mut gumball_package = Package::new("tests/assets/gumball-machine/");
        gumball_package.add_blueprint("gumball", gumball_blueprint);
        test_env.publish_package("gumball", gumball_package);
        test_env.new_component("gumball_comp", "gumball", "gumball");
        test_env.call_method("gumball_comp", GumballMethods::BuyGumball(dec!(15)));

        assert_eq!(test_env.get_current_account().amount_owned(test_env.get_token("gumball").unwrap()), Decimal::one());
    }
}