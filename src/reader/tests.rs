#[cfg(test)]
mod test {
    use crate::MyReader;
    use std::sync::mpsc::{channel, sync_channel};
    use std::thread::spawn;

    #[test]
    fn read_to_channel() {
        let (tx, rx) = sync_channel(1);
        let th = spawn(move || {
            let reader = MyReader::new("foo.txt", b'\n', tx).unwrap();
            reader.split_file();
        });
        for s in rx {
            println!("{}", s);
        }
        th.join();
    }
}
