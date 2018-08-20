pub fn raindrops(n: usize) -> String {
    if (n%3 == 0) && (n%5 == 0) && (n%7 == 0 ) {
        "PlingPlangPlong".to_string()
    } else if (n%3 == 0) && (n%5 == 0) {
        "PlingPlang".to_string()
    } else if (n%5 == 0) && (n%7 == 0) {
        "PlangPlong".to_string()
    } else if (n%3 == 0) && (n%7 == 0) {
        "PlingPlong".to_string()
    } else if n%3 == 0 {
        "Pling".to_string()
    } else if n%5 == 0 {
        "Plang".to_string()
    } else if n%7 == 0 {
        "Plong".to_string()
    } else {
        n.to_string()
    }
}
