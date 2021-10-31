pub fn decode_variants<T: std::io::Read>(data: &mut T) -> Option<u128> {
    // iterate take_util とかでもできるよ
    let mut sum = 0;
    let mut loop_count = 0;
    loop {
        let mut buf = [0; 1];
        let result = data.read_exact(&mut buf);
        if result.is_err() {
            return None;
        }
        // MSB は後続のバイトが続くかどうかの判定に使われる
        // 1 の場合、後続が続く
        let top = buf[0] & 0b10000000;
        let buf: u128 = (buf[0] & 0b01111111) as u128;
        // little endian
        let buf = buf << (7 * loop_count);
        sum += buf;
        loop_count += 1;
        if top != 0b10000000 {
            return Some(sum);
        }
    }
}
