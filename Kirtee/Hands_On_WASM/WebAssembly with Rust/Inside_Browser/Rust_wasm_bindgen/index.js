const rust = import('./wasm_example')

rust.then(func => {
    func.create_stuff()
    func.run_alert("JavaScript")
})