
pub mod public_module{

    fn public_module_private_function(){
        println!("public_module_private_function get called");
    }
    pub fn public_module_public_function(){
        public_module_private_function();
        println!("public_module_public_function get called\n");
    }
}

mod private_module{

    fn private_module_private_function(){
        println!("private_module_private_function get called");
    }
    pub fn private_module_public_function(){
        private_module_private_function();
        println!("private_module_public_function get called\n");
    }
}


use public_module::public_module_public_function;
use private_module::private_module_public_function;


fn main(){
    
    println!("--------------Method I of accessing functions------------");
    println!("Calling public Mudules functions");
    public_module_public_function();
    println!("Calling Private Modules Functions");
    private_module_public_function();

    println!("\n--------------Method II of accessing functions------------");
    println!("Calling public Mudules functions");
    public_module::public_module_public_function();
    println!("Calling Private Modules Functions");
    private_module::private_module_public_function();

}
