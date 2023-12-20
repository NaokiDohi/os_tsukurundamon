// OS依存のCの標準ライブラリを無効化。これからOSを開発するので当然使えないでゲス。
#![no_std]

use core::panic::PanicInfo;

// パニック時に呼ばれる。
// Cの標準ライブラリには、独自のパニックハンドラがあるがlibcは使えないので自力で実装する。
// 返り値を返さない(loop)のでnever型!を型宣言する
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    // printは、OS依存の関数なので使えないでゲス〜。
    // std out はOSによって提供される特殊なファイル記述子でゲス〜
    // println!("Hello, world!");
}
