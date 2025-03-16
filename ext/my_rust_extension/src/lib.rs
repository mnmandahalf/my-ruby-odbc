use libc::{c_char, c_long, c_short, c_void};
use magnus::{define_module, function, Error};
use std::ffi::CString;
use std::ptr;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("ODBC")?;
    module.define_module_function("execute", function!(execute, 0))?;
    Ok(())
}

#[link(name = "odbc")]
extern "C" {
    fn SQLAllocHandle(
        handle_type: c_short,
        input_handle: *mut c_void,
        output_handle: *mut *mut c_void,
    ) -> c_short;
    fn SQLSetEnvAttr(
        environment_handle: *mut c_void,
        attribute: c_long,
        value: *mut c_void,
        string_length: c_long,
    ) -> c_short;
    fn SQLConnect(
        handle: *mut c_void,
        server_name: *const c_char,
        name_length: c_short,
        user_name: *const c_char,
        user_length: c_short,
        auth: *const c_char,
        auth_length: c_short,
    ) -> c_short;
    fn SQLExecDirect(
        statement_handle: *mut c_void,
        statement_text: *const c_char,
        text_length: c_long,
    ) -> c_short;
    fn SQLFetch(statement_handle: *mut c_void) -> c_short;
    fn SQLGetData(
        statement_handle: *mut c_void,
        column_number: c_short,
        target_type: c_short,
        target_value: *mut c_void,
        buffer_length: c_long,
        out_length: *mut c_long,
    ) -> c_short;
    fn SQLFreeHandle(handle_type: c_short, handle: *mut c_void) -> c_short;
}

const SQL_SUCCESS: c_short = 0;
const SQL_HANDLE_ENV: c_short = 1;
const SQL_HANDLE_DBC: c_short = 2;
const SQL_HANDLE_STMT: c_short = 3;
const SQL_ATTR_ODBC_VERSION: c_long = 200;
const SQL_OV_ODBC3: c_long = 3;
const SQL_C_CHAR: c_short = 1;

pub fn execute() {
    let mut env_handle: *mut c_void = ptr::null_mut();
    let mut dbc_handle: *mut c_void = ptr::null_mut();
    let mut stmt_handle: *mut c_void = ptr::null_mut();
    let mut ret: c_short;

    unsafe {
        // Set up the environment
        ret = SQLAllocHandle(SQL_HANDLE_ENV, ptr::null_mut(), &mut env_handle);
        if ret != SQL_SUCCESS {
            panic!("SQLAllocHandle failed");
        }

        // Set ODBC version
        ret = SQLSetEnvAttr(
            env_handle,
            SQL_ATTR_ODBC_VERSION,
            SQL_OV_ODBC3 as *mut c_void,
            0,
        );
        if ret != SQL_SUCCESS {
            panic!("SQLSetEnvAttr failed");
        }

        // Connect to the database
        ret = SQLAllocHandle(SQL_HANDLE_DBC, env_handle, &mut dbc_handle);
        if ret != SQL_SUCCESS {
            panic!("SQLAllocHandle failed");
        }

        // Connect to the database
        let server_name = CString::new("DSN=PostgreSQL30").unwrap();
        let user_name = CString::new("postgres").unwrap();
        let auth = CString::new("password").unwrap();
        ret = SQLConnect(
            dbc_handle,
            server_name.as_ptr(),
            server_name.as_bytes().len() as c_short,
            user_name.as_ptr(),
            user_name.as_bytes().len() as c_short,
            auth.as_ptr(),
            auth.as_bytes().len() as c_short,
        );
        if ret != SQL_SUCCESS {
            panic!("SQLConnect failed");
        }

        // Execute a statement
        let statement_text = CString::new("SELECT * FROM test").unwrap();
        ret = SQLAllocHandle(SQL_HANDLE_STMT, dbc_handle, &mut stmt_handle);
        if ret != SQL_SUCCESS {
            panic!("SQLAllocHandle failed");
        }

        ret = SQLExecDirect(
            stmt_handle,
            statement_text.as_ptr(),
            statement_text.as_bytes().len() as c_long,
        );
        if ret != SQL_SUCCESS {
            panic!("SQLExecDirect failed");
        }

        // Fetch the data
        let mut column_number: c_short = 1;
        let buffer_length: c_long = 1024;
        let mut out_length: c_long = 0;
        let target_value: *mut c_void = ptr::null_mut();

        while SQLFetch(stmt_handle) == SQL_SUCCESS {
            let ret = SQLGetData(
                stmt_handle,
                column_number,
                SQL_C_CHAR,
                target_value,
                buffer_length,
                &mut out_length,
            );
            if ret == SQL_SUCCESS {
                let data_str = match target_value.is_null() {
                    true => "NULL".to_string(),
                    false => {
                        let data = CString::from_raw(target_value as *mut c_char);
                        data.to_str().unwrap().to_string()
                    }
                };
                println!("Column {}: {}", column_number, data_str);
                column_number += 1;
            } else {
                panic!("SQLGetData failed");
            }
        }

        // Clean up
        SQLFreeHandle(SQL_HANDLE_STMT, stmt_handle);
        SQLFreeHandle(SQL_HANDLE_DBC, dbc_handle);
        SQLFreeHandle(SQL_HANDLE_ENV, env_handle);
    }
}
