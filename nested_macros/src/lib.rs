#[macro_export]
macro_rules! print_feature {
    () => {
        #[cfg(feature = "my_feature")]
        macro_rules! feature_macro {
            () => {
                println!("I found the feature.");
            };
        };

        #[cfg(not(feature = "my_feature"))]
        macro_rules! feature_macro {
            () => {
                println!("I DID NOT find the feature.");
            };
        }

        feature_macro!()
    };
}
