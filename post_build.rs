use std::process::Command;

//fixme: これで生成された成果物で動作確認は行っていない
//fixme: macOS以外だとbuildできなくなる。OSを識別してコマンドを実行するか選べるようにする
fn main() {
    //todo: 生成するpluginの名前をプロジェクト名から取得できるようにする
    Command::new("sh").args(&["./osx_vst_bundler.sh", "./target/debug/wisper",  "target/release/plugin.dylib"])
        .status().unwrap();
}