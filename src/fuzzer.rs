use libfuzzer_sys::fuzz_target;
use pyo3::prelude::*;
use crate::utils::*;

#[no_mangle]
pub extern "C" fn PythonFuzzer(data: *const u8, size: usize) 
{
    // Convert data to a Python string
    let input_data = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, size)) };

    // Initialize Python interpreter
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sys = py.import("sys").unwrap();

    // Use set_item to set a value in the Python module
    sys.set_item("argv", input_data).unwrap();

    // Execute Python code with input data
    let result = py.run(input_data, None, None);

    if let Err(err) = result 
    {
        eprintln!("Python code execution error: {:?}", err);
    }
}

fuzz_target!(|data: &[u8]| 
{
    let input_data1 = generate_random_input(data.len());
    let input_data2 = generate_random_input_with_seed(data.len(), 0xdeadbeef);
    let input_data3 = generate_random_input_with_range(data.len(), 0x20, 0x7e);
    let input_data4 = generate_random_input_with_seed_and_range(data.len(), 0xdeadbeef, 0x20, 0x7e);
    let input_data5 = generate_random_input_with_range_and_chars(data.len(), 0x20, 0x7e, "abc");
    let input_data6 = generate_random_input_with_seed_and_range_and_chars(data.len(), 0xdeadbeef, 0x20, 0x7e, "abc");
   
    // Match the remainder of the division by 10
    match data.len() % 10 
    {
        0 => PythonFuzzer(input_data1.as_bytes().as_ptr(), input_data1.len()),
        1 => PythonFuzzer(input_data2.as_bytes().as_ptr(), input_data2.len()),
        2 => PythonFuzzer(input_data3.as_bytes().as_ptr(), input_data3.len()),
        3 => PythonFuzzer(input_data4.as_bytes().as_ptr(), input_data4.len()),
        4 => PythonFuzzer(input_data5.as_bytes().as_ptr(), input_data5.len()),
        5 => PythonFuzzer(input_data6.as_bytes().as_ptr(), input_data6.len()),
        _ => PythonFuzzer(input_data1.as_bytes().as_ptr(), input_data1.len()), // Default case
    }
});
