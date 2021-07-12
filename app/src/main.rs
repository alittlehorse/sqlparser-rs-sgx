// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::ffi::CString;
use std::slice;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn lexer(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
        sql: *const u8, sql_len: usize,ouput: *mut u8,ouput_len:*mut usize) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {

    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let input_string = String::from("SELECT * FROM customer WHERE id = 1 LIMIT 5\n");

    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut lexer_result = "".as_ptr() as *mut u8;
    let mut output_len:usize = 21;
    let mut len_ptr:*mut usize = &mut output_len;
    //let mut lexer_len:usize = 1;
    //let lexer_output = CString::new("").expect("CString::new failed").into_bytes_with_nul().as_mut_ptr();
    // let result = unsafe {
    //     lexer(enclave.geteid(),
    //                   &mut retval,
    //                   input_string.as_ptr() as * const u8,
    //                   input_string.len(),lexer_result)
    // };
    let result = unsafe {
        lexer(enclave.geteid(),
                      &mut retval,
                      input_string.as_ptr() as * const u8,
                      input_string.len(),lexer_result,len_ptr)
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {
            unsafe {
                println!("==========\n out of enclace:the output len is {}",&*len_ptr);
                let str_slice = unsafe {  slice::from_raw_parts(lexer_result,*len_ptr) };
                println!("=========='{:?}n",str_slice);
        };
        }
        ,
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] lexer success...");

    enclave.destroy();
}
