pub mod inner;
use crate::my_library;

pub fn call_secret(){
    my_library::secret_function();
    inner::secret_function();
}
