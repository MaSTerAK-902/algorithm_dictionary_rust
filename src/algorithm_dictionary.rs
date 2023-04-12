/// 値の交換
#[cfg(feature = "exchange_of_values")]
pub fn exchange_of_values (x: &mut i32, y: &mut i32){
    let temp = *x; 
    *x = *y;
    *y = temp;
}


/// 注意: この例は教育用であり、実際の用途には最適化やエラー処理が必要です。
/// また、このアルゴリズムは、クレジットカード番号の構造的な正当性を確認するだけであり、
/// その他の検証（有効期限、セキュリティコード、実際のアカウントの有効性など）は行われません。
/// 実際のシステムでは、これらの追加検証を行い、適切なセキュリティ対策を講じる必要があります。

/// 誤り検出符号
#[cfg(feature = "error_detcting_code")]
pub fn error_detecting_code (cash_num: &str) -> bool {
    let mut sum = 0; //合計値の初期化
    let mut odd_digit =cash_num.len() % 2 == 0; //偶数判定
    //cash_numの各文字に対してループを実行する。
    for c in cash_num.chars() {
        if let Some(digit) = c.to_digit(10){
            let mut doubled = digit * (1 + odd_digit as u32); //odd_digitが偶数の場合は０、奇数の場合は１に変換される。
            if doubled > 9 {
                doubled -= 9;
            }
            sum += doubled;
            odd_digit = !odd_digit;
        }else {
            return false; // 不正な文字が含まれている場合
        }
    }

    sum % 10 == 0
}

///XOR暗号化
#[cfg(feature = "cryptosystem")]
pub fn cryptosystem(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|byte| byte ^ key).collect()
    //暗号化：cryptosystem(plane_message, secret_key);
    //復号化：cryptosystem(&String::from_utf8_lossy(&encrypted), secret_key);
    //復号化は一度、utf-8からbyteに変換しているので、再度表示する場合はもう一度utf-8に変換しなければいけない
}

/// 安定的な結婚問題
pub fn stable_marriage_problem (men_pref: &Vec<Vec<usize>>, women_pref: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = men_pref.len(); // 男性（および女性）の数

    // 女性のパートナーを初期化
    let mut women_partner: Vec<Option<usize>> = vec![None; n];

    // 男性のパートナーを初期化
    let mut men_partner: Vec<usize> = vec![0; n];

    // 男性の次のプロポーズのインデックスを初期化
    let mut men_next: Vec<usize> = vec![0; n];

    // 空いている男性のリストを作成
    let mut free_men = (0..n).collect::<Vec<_>>();

    // 空いている男性がいる間
    while let Some(man) = free_men.pop() {
        // 男性がプロポーズする女性のインデックスを取得
        let woman = men_pref[man][men_next[man]];

        // 男性の次のプロポーズのインデックスを更新
        men_next[man] += 1;

        // 女性がすでにパートナーがいるかどうかをチェック
        if let Some(current_man) = women_partner[woman] {
            // 新しいプロポーザーが現在のパートナーよりも好まれる場合
            if women_pref[woman].iter().position(|&m| m == man).unwrap() < women_pref[woman].iter().position(|&m| m == current_man).unwrap() {
                // 現在のパートナーを空いている男性リストに戻す
                free_men.push(current_man);

                // 新しいプロポーザーと女性をペアにする
                men_partner[man] = woman;

                // 女性のパートナーを更新
                women_partner[woman] = Some(man);
            } else {
                // 新しいプロポーザーを空いている男性リストに戻す
                free_men.push(man);
            }
        } else {
            // 男性と女性をペアにする
            men_partner[man] = woman;

            // 女性のパートナーを更新
            women_partner[woman] = Some(man);
        }
    }

    men_partner
}
