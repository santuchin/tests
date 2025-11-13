use std::ffi::CString;

fn main() {
    let path = CString::new("archivo.txt").unwrap();

    // abrir archivo
    let fd = unsafe { open(path.as_ptr(), O_CREAT | O_WRONLY, S_IRUSR | S_IWUSR) };
    if fd < 0 {
        panic!("error al abrir archivo");
    }

    // escribir
    let buf = b"Hola desde linux-raw-sys\n";
    unsafe { write(fd, buf.as_ptr() as _, buf.len()) };

    // cerrar
    unsafe { close(fd) };
}
