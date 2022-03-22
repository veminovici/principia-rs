// Error Handling in FFI
// https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html

pub enum DatabaseError {
    IsReadonly,
    IOError(std::io::Error),
    FileCorrupted(String),
}

impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> Self {
        match e {
            DatabaseError::IsReadonly => 1,
            DatabaseError::IOError(_) => 2,
            DatabaseError::FileCorrupted(_) => 3,
        }
    }
}

pub mod c_api {
    use super::DatabaseError;

    #[no_mangle]
    pub extern "C" fn db_error_description(
        e: *const DatabaseError
        ) -> *mut libc::c_char 
    {

        let error: &DatabaseError = unsafe {
            &*e
        };

        let error_str: String = match error {
            DatabaseError::IsReadonly => {
                format!("cannot write to read-only database")
            }
            DatabaseError::IOError(e) => {
                format!("I/O Error: {}", e)
            }
            DatabaseError::FileCorrupted(s) => {
                format!("File corrupted, run repair: {}", &s)
            }
        };

        let c_error = unsafe {
             let malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;

             if malloc.is_null() {
                 return std::ptr::null_mut();
             }

             let src = error_str.as_bytes().as_ptr();
             std::ptr::copy_nonoverlapping(src, malloc, error_str.len());
             std::ptr::write(malloc.add(error_str.len()), 0);

             malloc as *mut libc::c_char
        };

        c_error
    }
}

fn main() {
    println!("Hello, world!");
}
