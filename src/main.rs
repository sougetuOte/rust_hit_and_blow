// ヒットアンドブローゲーム
// メインモジュール

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
        
        // ゲーム継続の確認
        print!("続けますか？ (y/n): ");
        let input = io::get_input("");
        
        // 'n'が入力された場合のみループを抜ける
        if input == "n" {
            io::display_game_end();
            break; // ループを抜ける
        }
    }
    
    // ゲーム終了後のメッセージ
    println!("ゲームが終了しました。");
}
