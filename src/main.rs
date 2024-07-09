use owns_buffer::OwnsBuffer;

fn do_stuff(buffer: &mut [u8]) {
    buffer[0] = 1;
}

fn main() {
    let mut buffer: OwnsBuffer<2, u8> = OwnsBuffer::default();
    do_stuff(buffer.get_mut());
    println!("{:?}", buffer.get());
}
