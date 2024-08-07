fn main() {
    #[derive(Debug)]
    enum Storage {
        HDD { size: u32, rpm: u32},
        SSD(u32),
    }

    let s = Storage::HDD {
        size: 20418,
        rpm: 7200,
    };

    println!("{:?}", s);
    println!("{:#?}", s);
}