pub mod test_a;
pub mod test_b;
//testフォルダはこいつと同じ名前じゃないと実行できない

pub fn run() {
    test_a::func_a();
    test_b::func_b();
}