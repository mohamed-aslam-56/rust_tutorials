use re_export::{my_library,outer_lib};
fn main(){
    my_library::secret_function();
    outer_lib::call_secret();
}