use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    // 중위 표기법으로 된 수식 입력
    let stdin = stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // 연산자 우선순위를 hashmap으로 저장
    let isp = HashMap::from([('+', 0), ('-', 0), ('*', 1), ('/', 1), ('(', -1)]);

    let mut opers: Vec<char> = Vec::new(); // 입력된 수식의 연산자를 담는 스택
    let mut postfix: Vec<String> = Vec::new(); // 입련된 수식을 후위표기법으로 바꾼 결과를 담는 스택
    let mut result: Vec<f64> = Vec::new(); // 수식의 연산 결과를 담는 스택

    let mut ch1 = true; // 중위 표기법대로 제대로 입력했는지 확인하는 부울대수
    let mut ch2 = !ch1;

    // 입력된 수식을 띄어쓰기 기준으로 나누고 각각 반복문으로 처리
    for s in input.split_whitespace() {
        if let Ok(_) = s.parse::<f64>() {
            ch2 = ch1;
            ch1 = false;
            postfix.push(s.to_string()); // 실수로 변환 가능하면 postfix에 push
            continue;
        }
        // 띄어쓰기를 하지 않은 경우 에러처리 ex) 1+2, 2 + 3*4
        if s.chars().count() > 1 {
            println!("올바르게 입력해주세요!");
            return;
        }
        let c = s.chars().next().unwrap(); // 연산자를 c로 저장
        if c == '(' { opers.push(c); }
        else if c == ')' {
            loop {
                match opers.pop() {
                    Some('(') => break,
                    Some(x) => postfix.push(x.to_string()),
                    None => break,
                }
            }
        } else if ['+', '-', '*', '/'].contains(&c) {
            ch2 = ch1;
            ch1 = true;
            while opers.len() > 0 {
                if isp[opers.last().unwrap()] >= isp[&c] {
                    postfix.push(opers.pop().unwrap().to_string());
                } else { break; }
            }
            opers.push(c);
        } else {
            // 지원하지 않는 연산자 입력시 에러처리 ex) 1 @ 2
            println!("올바르게 입력해주세요!");
            return;
        }
        if ch1 == ch2 {
            println!("올바르게 입력해주세요!");
            return;
        }
    }

    if ch1 {
        println!("올바르게 입력해주세요!");
        return;
    }

    if opers.len() > 0 {
        opers.reverse();
        for &o in opers.iter() {
            postfix.push(o.to_string());
        }
    }

    for c in postfix.iter() {
        if let Ok(x) = c.parse::<f64>() {
            result.push(x);
        } else {
            let tmp = c.chars().next().unwrap();
            let k;
            match tmp {
                '+' => {
                    k = result.pop().unwrap() + result.pop().unwrap();
                    result.push(k);
                },
                '-' => {
                    k = - result.pop().unwrap() + result.pop().unwrap();
                    result.push(k);
                },
                '*' => {
                    k = result.pop().unwrap() * result.pop().unwrap();
                    result.push(k);
                },
                '/' => {
                    k = 1.0 / (result.pop().unwrap() / result.pop().unwrap());
                    result.push(k);
                },
                _ => (),
            }
        }
    }

    println!("결과: {}", result.pop().unwrap());
}
