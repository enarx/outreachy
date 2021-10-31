extern crate wasmi;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};
fn main() -> Result<(), Box<Error>> {
    let mut buffer = Vec::new();
    {
        let mut f = File::open("../fundamentals/add.wasm")?;
        f.read_to_end(&mut buffer)?;
    }
    let module = wasmi::Module::from_buffer(buffer)?;

    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("Failed to instantiate WASM module")
        .assert_no_start();

        let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(42));
    args.push(RuntimeValue::from(1));

    let result: Option<RuntimeValue> = instance.invoke_export("add", &args, &mut NopExternals)?;

    match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your addition was {}", v);
        }
        Some(_) => {
            println!("Got a value of an unexpected data type");
        }
        None => {
            println!("Failed to get a result from wasm invocation");
        }
    }
    Ok(())
}