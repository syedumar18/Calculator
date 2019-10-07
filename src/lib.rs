mod Calculator{
    mod Calculator_functions{
        use std::io;
        pub fn Add(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}", x+y);

        }
        pub fn Subtract(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}", x-y);

        }
        pub fn Multiplication(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}", x*y);

        }
        pub fn Divide(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

            println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:i32= y.trim().parse().unwrap();

            println!("{}", x/y);

        }
    }
    mod Power_functions{
        use std::io;
        pub fn square_function(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

        
            println!("{}", x*x);


        }
        pub fn cube_function(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:i32= x.trim().parse().unwrap();

        
            println!("{}", x*x*x);


        }
        pub fn power_functions(){
            println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:f32= x.trim().parse().unwrap();

            println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:f32= y.trim().parse().unwrap();

            println!("{}",x.powf(y));
 
           

        }
    }
}