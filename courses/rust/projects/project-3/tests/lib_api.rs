mod macro_tests {
    #[macro_export]
    macro_rules! my_vec {
    ($( $x: expr );* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
            temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
    #[test]
    fn test_vec_macro() {
        let my_vec = my_vec!(1; 2; 3; 4);
        println!("{:?}", my_vec);

        let my_vec = my_vec!["hello"; "world"];

        println!["{:?}", my_vec];
    }
}
