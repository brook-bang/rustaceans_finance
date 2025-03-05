fn main() {
    let portfolio_risk_scores = vec![0.8,0.6,0.9,0.5,0.7];
    let risk_threshold = 0.7;

    let mut high_risk_assets = 0;

    for &risk_score in portfolio_risk_scores.iter() {
        if risk_score > risk_threshold {
            high_risk_assets += 1;
        }
    }

    if high_risk_assets == 0 {
        println!("投资组合风险水平低，没有高风险资产。");
    } else if high_risk_assets <= 2 {
        println!("投资组合风险水平中等，有少量高风险资产。");
    } else {
        println!("投资组合风险水平较高，有多个高风险资产");
    }

}
