pub fn loan_test() {
    equ_principal_month(2000000., 30, 0.0315);
    equ_profit_month(2000000., 30, 0.0315)
}


// 等额本金按照年还款
// n: 贷款总额， y：贷款时间，p：贷款利率
pub fn equ_principal_year(n: f32, y: i32, p: f32) {
    // 第一年
    // let mut profit = n * p;
    // let mut principal = n / (y as f32);
    // 第二年
    // profit = (n - principal) * p;
    // principal = principal;

    // 第三年
    // profit = (n - principal - principal) * p;

    // 以此类推
    // profit = (n - principal) * p + (n - principal * 2.) * p
    //+ .... +
    // + (n - principal * (y as f32)) * p;

    let mut principal_per_year = n / (y as f32);
    let mut profit_count: f32 = 0.;
    for index in 0..y {
        profit_count += (n - principal_per_year * (index as f32)) * p;
    }
    let count = profit_count + n;
    let count_average_per_month = profit_count / (y as f32 * 12.) + principal_per_year / 12.;
    let profit_average_per_month = profit_count / (y as f32 * 12.);
    println!("等额本金按照年还款: count:{}, profit_count:{}, count_average_per_month:{}, profit_average_per_month:{}",
             count, profit_count, count_average_per_month, profit_average_per_month);
}

// 等额本金按照月还款计算
// n: 贷款总额， y：贷款时间，p：贷款利率
pub fn equ_principal_month(n: f32, y: i32, p: f32) {
    let r = p / 12.;
    let mut principal_per_month = n / (y as f32 * 12.);
    let mut profit_count: f32 = 0.;
    for index in 0..(y * 12) {
        profit_count += (n - principal_per_month * (index as f32)) * r;
    }
    let count = profit_count + n;
    let count_average_per_month = profit_count / (y as f32 * 12.) + principal_per_month;
    let profit_average_per_month = profit_count / (y as f32 * 12.);
    println!("等额本金按照月还款: count:{}, profit_count:{}, count_average_per_month:{}, profit_average_per_month:{}",
             count, profit_count, count_average_per_month, profit_average_per_month);
}


// 等额本息按照月还款计算
// n: 贷款总额， y：贷款时间，p：贷款利率
pub fn equ_profit_month(n: f64, y: i32, p: f64) {
    let r = p / 12.;
    let count_month = y * 12;
    let base: f64 = r + 1.;
    let count_average_per_month = n * r * base.powf(count_month as f64) / (base.powf(count_month as f64) - 1.);
    let count = count_average_per_month * (count_month as f64);
    let profit_count = count - n;
    let profit_average_per_month = profit_count / (count_month as f64);
    println!("等额本息按照月还款: count:{}, profit_count:{}, count_average_per_month:{}, profit_average_per_month:{}",
             count, profit_count, count_average_per_month, profit_average_per_month);
}