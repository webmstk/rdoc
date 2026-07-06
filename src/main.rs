use cargo_rdoc::rdoc;

fn main() {
    let args = std::env::args().skip(2).collect();
    rdoc(args);
}
