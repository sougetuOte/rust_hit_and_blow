//! ヒットアンドブローゲーム
//! メインモジュール

// 自作モジュールをインポート
mod game; // Game運用などのモジュール
mod io; // 入出力処理のモジュール

// 自作モジュールから必要な要素をインポート
//gameモジュールからGame構造体をインポート
use game::Game;

/// ヒットアンドブローゲームのメイン関数
fn main() {
    // ゲームのタイトルとルールを表示
    // ioモジュールの関数を使用
    io::display_title();
    io::display_rules();
    
    // ゲームループ
    loop {
        // ゲーム開始のメッセージを表示
        io::display_game_start();
        
        // 新しいゲームを作成
        // new()メソッドを使用してGame構造体のインスタンスを作成
        let mut game = Game::new();
        
        // デバッグ用：正解を表示（本番環境ではコメントアウト）
        // println!("デバッグ用 - 正解: {:?}", game.get_answer());
        
        // ゲームが終了するまでループ
        // is_game_over()メソッドを使用してゲームの状態を確認
        // game.guess()メソッド内で、is_game_overフラグを更新
        //回数オーバーまたは正解の場合、is_game_overフラグがtrueになる
        while !game.is_game_over() {
            // 現在の試行回数を取得
            let current_attempt = game.get_attempts();
            let max_attempts = game.get_max_attempts();
            
            // 数字入力のプロンプトを表示
            io::display_guess_prompt(current_attempt, max_attempts);
            
            // ユーザーからの入力を取得
            let input = io::get_input("");
            
            // 入力を解析
            match Game::parse_input(&input) {
                Some(guess) => {
                    // 予想を判定
                    let result = game.guess(&guess);
                    
                    // 結果を表示
                    io::display_result(result.hits, result.blows, current_attempt + 1, max_attempts);
                    
                    // 正解の場合
                    if result.is_correct {
                        io::display_correct();
                    }
                    // ゲームオーバーの場合
                    else if result.is_game_over {
                        io::display_game_over(game.get_answer());
                    }
                },
                None => {
                    // 入力エラーの場合
                    io::display_input_error();
                }
            }
        }
        
        // ゲーム終了かどうかの確認。
        // y/nで質問し、yならループを続け、nならゲームを終了する。その他の場合警告し、もう一度質問する。
        loop {
            // ゲーム継続の確認
            print!("続けますか？ (y/n): ");
            // ioモジュールの関数を使用して、ユーザーからの入力を取得
            let input = io::get_input("").chars().take(1).collect::<String>();
            
            match input.as_str() {
                "y" => break, // 続ける場合はループを抜ける
                "n" => {
                    io::display_game_end();
                    return; // メインループを終了
                },
                _ => {
                    // 無効な入力の場合は警告を表示
                    println!("無効な入力です。'y'または'n'を入力してください。");
                }
            }
        }
    }
    
}
