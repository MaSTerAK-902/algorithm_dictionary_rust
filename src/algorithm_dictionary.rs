//値の交換
#[cfg(feature = "exchange_of_values")]
pub fn exchange_of_values (x: &mut i32, y: &mut i32){
    let temp = *x; 
    *x = *y;
    *y = temp;
}

/*
注意: この例は教育用であり、実際の用途には最適化やエラー処理が必要です。
また、このアルゴリズムは、クレジットカード番号の構造的な正当性を確認するだけであり、
その他の検証（有効期限、セキュリティコード、実際のアカウントの有効性など）は行われません。
実際のシステムでは、これらの追加検証を行い、適切なセキュリティ対策を講じる必要があります。
 */

//誤り検出符号
#[cfg(feature = "error_detcting_code")]
pub fn error_detecting_code (cash_num: &str) -> bool {
    let mut sum = 0; //合計値の初期化
    let mut odd_digit =cash_num.len() % 2 == 0; //偶数判定

    /*
    cash_numの各文字に対してループを実行する。

    */
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