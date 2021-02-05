use crate::io_module::IO;

pub fn login(io: &mut IO) {
    let x = String::from("moi");

    io.println(&x);
    io.println(&x);
}
