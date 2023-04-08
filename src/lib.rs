pub fn unique(list: &mut Vec<f32>) {
    let mut length = list.len();
    let mut pointer = 1;
    loop {
        if pointer >= length {
            break;
        }
        if list[pointer - 1] == list[pointer] {
            list.remove(pointer);
            length -= 1;
        }
        pointer += 1;
    }
}