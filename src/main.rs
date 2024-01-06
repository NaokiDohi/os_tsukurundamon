// OS依存のCの標準ライブラリを無効化。これからOSを開発するので当然使えないでゲス。
#![no_std]
// デフォルトのエントリーポイントを削除する。同時にmain関数を消去する。
#![no_main]

use core::panic::PanicInfo;

// パニック時に呼ばれる。
// Cの標準ライブラリには、独自のパニックハンドラがあるがlibcは使えないので自力で実装する。
// 返り値を返さない(loop)のでnever型!を型宣言する
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// エントリポイントを_start関数で上書き。no_mangleはユニークなシンボルをコンパイラが生成するのを防ぐ。
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}