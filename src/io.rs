//! CUI 入力・表示ユーティリティ
//! もう少し大きなプログラムであれば、リテラルは外出しても良いかもしれないが、
//! この程度のプログラムでは、リテラルを直接書いても良いと思う。

use std::io::{self, Write};

/// ユーザーからの入力を取得する
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // バッファをフラッシュして即座に表示
    // 標準出力に書き込まれたデータを即座に表示するためにflush()メソッドを使用
    // これをやらないと、バッファリングのために出力が遅れることがある
    io::stdout().flush().unwrap();
    
    // ユーザーからの入力を受け取る
    let mut input = String::new(); // 入力用の文字列を初期化
    io::stdin().read_line(&mut input).unwrap(); // 標準入力から1行読み込む
    
    // 入力を小文字にして、trimで空白を削除
    input.trim().to_lowercase()
}

/// ゲームのタイトルを表示する
pub fn display_title() {
    println!("ヒットアンドブロー\nHit and Blow\n---------------------\n");
}

/// ゲームのルールを表示する
pub fn display_rules() {
    let game_rules = "
        ルール
        1. 4桁の数字を当てるゲームです。
        2. 各桁は1から6までの数字で、同じ数字は使いません。
        3. ユーザーが入力した数字と正解の数字を比較し、ヒットとブローを教えます。
        4. ヒットは正しい位置にある数字、ブローは正しい数字だが位置が違うものです。
        5. ゲームは10回まで続きます。
    ";
    
    println!("{}", game_rules);
}

/// ゲームの結果を表示する
pub fn display_result(hits: u8, blows: u8, attempts: u8, max_attempts: u8) {
    println!("結果: {}ヒット, {}ブロー", hits, blows);
    println!("試行回数: {}/{}", attempts, max_attempts);
}

/// 正解時のメッセージを表示する
pub fn display_correct() {
    println!("正解です！おめでとうございます！");
}

/// ゲームオーバー時のメッセージを表示する
pub fn display_game_over(answer: &[u8]) {
    println!("ゲームオーバー！");
    print!("正解は: ");
    // 正解の数字を表示
    // answer(Vec<u8>)をループして、各数字を表示
    for digit in answer {
        print!("{}", digit);
    }
    println!();
}

/// 入力エラー時のメッセージを表示する
pub fn display_input_error() {
    println!("無効な入力です。1から6までの数字を4つ入力してください。同じ数字は使えません。");
}

/// ゲーム開始のメッセージを表示する
pub fn display_game_start() {
    println!("ゲームを開始します...");
}

/// ゲーム終了のメッセージを表示する
pub fn display_game_end() {
    println!("ゲームを終了します。");
}

/// 数字入力のプロンプトを表示する
pub fn display_guess_prompt(attempt: u8, max_attempts: u8) {
    println!("\n試行 {}/{}", attempt + 1, max_attempts);
    print!("4桁の数字を入力してください (1-6): ");
    io::stdout().flush().unwrap();
}
