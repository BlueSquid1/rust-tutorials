#[derive(Debug)]
struct RingBuffer {
    start: usize,
    len: usize,
    buffer: Vec<i32>,
}

impl RingBuffer {
    fn new(size: usize) -> Self {
        return Self {
            start: 0,
            len: 0,
            buffer: vec![0; size],
        };
    }

    fn size(&self) -> usize {
        return self.buffer.capacity();
    }

    fn push(&mut self, value: i32) {
        if self.len >= self.size() {
            // overwrite oldest value
            self.start = (self.start + 1) % self.size();
            let next_free_index = (self.start + self.len - 1) % self.size();
            self.buffer[next_free_index] = value;
            return;
        }

        let next_free_index = (self.start + self.len) % self.size();
        self.buffer[next_free_index] = value;
        self.len += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        if self.len <= 0 {
            return None;
        }

        let last_index = (self.start + self.len - 1) % self.size();
        self.len -= 1;
        return Some(self.buffer[last_index]);
    }
}

fn main() {
    let mut x = RingBuffer::new(3);
    x.push(2);
    x.push(3);
    x.push(4);
    x.push(5);

    // Can retrieve values from enums like this:
    match x.pop() {
        Some(value) => println!("{value}"), // happy path
        None => (),                         // unhappy path
    }

    // This is doing the same thing. just syntax sugar:
    if let Some(value) = x.pop() {
        println!("{value}"); //only the happy path
    }

    // also syntax sugar but must use a return or panic in the unhappy with.
    let Some(value) = x.pop() else {
        panic!("didn't expect it to failure here")
    };
    println!("{value}");

    // For debugging can also just print the member variables in the Option enum
    println!("{:?}", x.pop());

    x.push(6);
    x.push(7);
    println!("{:?}", x.pop());
    println!("{:?}", x.pop());
    //println!("{x:#?}");
}
