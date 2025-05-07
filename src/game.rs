// ヒットアンドブローゲームのロジック

use rand::seq::SliceRandom;
use rand::thread_rng;

/// ゲームの状態を管理する構造体
pub struct Game {
    /// 正解の数字（4桁）
    /// Vec<u8>は、動的な長さの配列を表す Vector 型
    /// u8 は、0から255までの整数を表す型
    /// u8以外の整数型としては、i8, i16, i32, i64, u16, u32, u64 などがある
    /// 実数を扱う型としては、f32, f64 などがある
    /// ここでlet mutが付いてないのは、構造体のフィールドとして定義しているから
    answer: Vec<u8>,
    /// 現在の試行回数
    attempts: u8,
    /// 最大試行回数
    max_attempts: u8,
    /// ゲームが終了したかどうか
    is_game_over: bool,
}

/// ゲームの結果を表す構造体
pub struct GuessResult {
    /// 正しい位置にある数字の数（ヒット）
    pub hits: u8,
    /// 正しい数字だが位置が違う数字の数（ブロー）
    pub blows: u8,
    /// 正解かどうか
    pub is_correct: bool,
    /// ゲームオーバーかどうか
    pub is_game_over: bool,
}

//// ゲームのロジックを管理する構造体
/// implは、構造体にメソッドを追加するためのキーワード
/// classのようなもの
impl Game {
    /// 新しいゲームを作成する
    pub fn new() -> Self {
        // 1から6までの数字を使って、重複のない4桁の数字を生成
        let mut numbers = vec![1, 2, 3, 4, 5, 6];
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);
        
        // 最初の4つの数字を取得
        let answer = numbers.into_iter().take(4).collect();
        
        Game {
            answer,
            attempts: 0,
            max_attempts: 10,
            is_game_over: false,
        }
    }
    
    /// 現在の試行回数を取得
    pub fn get_attempts(&self) -> u8 {
        self.attempts
    }
    
    /// 最大試行回数を取得
    pub fn get_max_attempts(&self) -> u8 {
        self.max_attempts
    }
    
    /// ゲームが終了したかどうかを取得
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }
    
    /// 正解を取得（デバッグ用）
    pub fn get_answer(&self) -> &Vec<u8> {
        &self.answer
    }
    
    /// プレイヤーの予想を判定する
    pub fn guess(&mut self, guess: &[u8]) -> GuessResult {
        // 試行回数を増やす
        self.attempts += 1;
        
        // ヒットとブローをカウント
        let mut hits = 0;
        let mut blows = 0;
        
        // ヒットをカウント（位置と数字が一致）
        for i in 0..4 {
            if guess[i] == self.answer[i] {
                hits += 1;
            }
        }
        
        // ブローをカウント（数字は一致するが位置が異なる）
        for i in 0..4 {
            for j in 0..4 {
                // 同じ位置の数字はカウントしない
                // guess[i]がself.answer[j]と一致するが、iとjが異なる場合
                if i != j && guess[i] == self.answer[j] {
                    blows += 1;
                }
            }
        }
        
        // 正解かどうかを判定
        let is_correct = hits == 4;
        
        // 残りの試行回数を計算
        let remaining_attempts = self.max_attempts - self.attempts;
        
        // ゲームオーバーかどうかを判定
        // 正解した場合、または試行回数が0になった場合 is_game_overをtrueにする
        if is_correct || remaining_attempts == 0 {
            self.is_game_over = true;
        }
        
        // GuessResult構造体を返す
        GuessResult {
            hits,
            blows,
            is_correct,
            is_game_over: self.is_game_over,
        }
    }
    
    /// 入力された文字列を数字の配列に変換する
    /// 変換できない場合はNoneを返す
    pub fn parse_input(input: &str) -> Option<Vec<u8>> {
        // 入力が4文字であることを確認
        if input.len() != 4 {
            return None;
        }
        
        let mut result = Vec::with_capacity(4);
        
        // 各文字を数字に変換
        for c in input.chars() {
            // 文字が1-6の範囲内の数字であることを確認
            if let Some(digit) = c.to_digit(10) {
                if digit >= 1 && digit <= 6 {
                    result.push(digit as u8);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        
        // 重複がないことを確認
        for i in 0..4 {
            for j in (i+1)..4 {
                if result[i] == result[j] {
                    return None;
                }
            }
        }
        
        Some(result)
    }
}

/// テスト用のモジュール
/// cfg(test)は、テストコードをコンパイルするための条件を指定するための属性
/// これにより、テストコードは通常のビルドには含まれず、テスト実行時のみコンパイルされる
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_input_valid() {
        let input = "1234";
        let result = Game::parse_input(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_parse_input_invalid_length() {
        let input = "123";
        let result = Game::parse_input(input);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_input_invalid_range() {
        let input = "1237";
        let result = Game::parse_input(input);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_input_duplicate() {
        let input = "1231";
        let result = Game::parse_input(input);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_guess_all_hit() {
        let mut game = Game::new();
        let answer = game.get_answer().clone();
        let result = game.guess(&answer);
        assert_eq!(result.hits, 4);
        assert_eq!(result.blows, 0);
        assert!(result.is_correct);
    }
    
    #[test]
    fn test_guess_some_hit_some_blow() {
        let mut game = Game {
            answer: vec![1, 2, 3, 4],
            attempts: 0,
            max_attempts: 10,
            is_game_over: false,
        };
        
        let guess = vec![1, 3, 2, 5];
        let result = game.guess(&guess);
        assert_eq!(result.hits, 1);
        assert_eq!(result.blows, 2);
        assert!(!result.is_correct);
    }
}
