extern crate mini_v8;

use mini_v8::{Array, Function, MiniV8, Object, Value};

fn main() {
    // A `MiniV8` is a V8 context that can execute JavaScript.

    //the doc says the data bridge of uint8Array is not implemented :(

    //and not similar to offical c api and deno rust v8 project.
    //this miniV8 look to use Isolate context all the way?
    //look ffi.c context_new()
    let mv8 = MiniV8::new();

    //case 1, rust manipulate
    let t: Value = mv8.eval("let variable = 123;").expect("error");
    println!(" {:?}", t);

    //js variable is exposed to rust
    if let mini_v8::Value::Number(num) = mv8.eval("variable").expect("error") {
        println!("variable's num : {:?}", num);
    };

    //get the js global object and set global.rust_test to 'rest_test_str'
    let global = mv8.global();
    let rust_test_str : String = global.get("rust_test").expect("error");
    println!("rest_test_str : {:?}", rust_test_str);//this should be undefined

    let res  = global.set("rust_test", "hello rust").expect("error");

    let rust_test_str : String = global.get("rust_test").expect("error");
    println!("rest_test_str2 : {:?}", rust_test_str);

    let res : String = mv8.eval("rust_test").expect("error~");
    println!(" {:?}", res);

    //case 2, rust pass value to rust
    let u8array: Object = mv8.eval(r#"
        let u8array = new Uint8Array(6);
        u8array[0] = 'h'.charCodeAt();
        u8array[1] = 'e'.charCodeAt();
        u8array[2] = 'l'.charCodeAt();
        u8array[3] = 'l'.charCodeAt();
        u8array[4] = 'o'.charCodeAt();
        u8array[5] = '!'.charCodeAt();
        u8array
    "#).expect("error");
    println!("u8array : {:?}", u8array);
//    let value0 : mini_v8::Value= u8array.get("0").expect("error");
//    println!(" value 0 : {:?}", value0);
    u8array.set(5, 0x7eu8).expect("error~");
    println!("u8array modified : {:?}", u8array);

    let u8array_toString: String = mv8.eval("String.fromCharCode.apply(null,u8array)").expect("error");
    println!("u8array.toString : {:?}", u8array_toString);

    //pass value to v8
    let object: Object = mv8.create_object();

    //note: we can implement a [u8] to js Uint8Array for better use
    let arr : Array = mv8.create_array();
    arr.set(0, 119).expect("e");
    arr.set(1, 111).expect("e");
    arr.set(2, 114).expect("e");
    arr.set(3, 108).expect("e");
    arr.set(4, 100).expect("e");
    global.set("array_rust", arr.clone()).expect("e");
    let array_rust_toString: String = mv8.eval("array_rust.toString()").expect("error");
    println!("array_rust.toString : {:?}", array_rust_toString);
    let u8array_rust_toString: String = mv8.eval("let u8array_rust = new Uint8Array(array_rust); u8array_rust.toString()").expect("error");
    println!("u8array_rust_toString.toString : {:?}", u8array_rust_toString);
    let u8array_rust_Type : String = mv8.eval("Object.prototype.toString.call(u8array_rust)").expect("e");
    println!("type of u8array_rust : {:?}", u8array_rust_Type);
    let u8array_rust_toString: String = mv8.eval("String.fromCharCode.apply(null,u8array_rust)").expect("error");
    println!("u8array.toString : {:?}", u8array_rust_toString);

    //case 3, bind rust function to js
    //first, declare rust function and bind it to V8 and get representation of mini_v8::Function()
    let rust_function: mini_v8::Function = mv8.create_function(|inv| {
        let (a, b): (f64, f64) = inv.args.into(inv.mv8)?;
        Ok(a + b)
    });

    //second, global.rust_function = our rust function
    global.set("rust_function", rust_function.clone()).expect("error");

    //third, call our rust function in V8
    let rust_function_result: String = mv8.eval("rust_function(5,6)").expect("error");
    println!("rust_function_result : {:?}", rust_function_result);

    //case 4, bind js function to rust
    //first, declare a js function and pass to rust
    let js_function :mini_v8::Function= mv8.eval("(a, b) => rust_test + ' ' + (a + b)").expect("error~");
    //second, call it
    let js_function_result: mini_v8::Value = js_function.call((1, 2)).expect("error");
    println!("js_function_result : {:?}", js_function_result);
}
