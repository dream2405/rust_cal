use std::io::{stdin, BufRead};
use std::collections::HashMap;

enum Term {
    Operand(f64),
    Operator(char),
}

fn main() {
    // 중위 표기법으로 된 수식 입력
    let input = stdin().lock().lines().next().unwrap().unwrap();
    // 연산자 우선순위를 hashmap으로 저장
    let isp = HashMap::from([('+', 0), ('-', 0), ('*', 1), ('/', 1), ('(', -1)]);

    let mut opers: Vec<char> = Vec::new(); // 입력된 수식의 연산자를 담는 스택
    let mut postfix: Vec<Term> = Vec::new(); // 입련된 수식을 후위표기법으로 바꾼 결과를 담는 스택
    let mut result: Vec<f64> = Vec::new(); // 수식의 연산 결과를 담는 스택

    let mut ch1 = true; // 중위 표기법대로 제대로 입력했는지 확인하는 부울대수
    let mut ch2;

    // 입력된 수식을 띄어쓰기 기준으로 나누고 각각 반복문으로 처리
    for s in input.split_whitespace() {
        ch2 = ch1;
        if let Ok(x) = s.parse::<f64>() {
            postfix.push(Term::Operand(x)); // 실수로 변환 가능하면 postfix에 push
            ch1 = false;
            continue;
        }
        // 띄어쓰기를 하지 않은 경우 에러처리 ex) 1+2, 2 + 3*4
        if s.chars().count() > 1 {
            println!("올바르게 입력해주세요!");
            return;
        }
        let c = s.chars().next().unwrap(); // 연산자를 c로 저장
        if c == '(' {
            opers.push(c);
            ch2 = !ch1;
        }
        else if c == ')' {
            loop {
                // (가 나올때까지 opers의 모든 연산자를 빼내고 postfix에 push
                match opers.pop() {
                    Some(x) => {
                        if x == '(' {
                            break;
                        }
                        postfix.push(Term::Operator(x))
                    },
                    _ => (),
                }
            }
            ch2 = !ch1;
        } else if ['+', '-', '*', '/'].contains(&c) {
            ch1 = true;
            while opers.len() > 0 {
                if isp[opers.last().unwrap()] >= isp[&c] {
                    // opers의 마지막 연산자가 입력된 연산자보다 우선순위가 같거나 높다면 pop하고 postfix에 push
                    postfix.push(Term::Operator(opers.pop().unwrap()));
                } else { break; }
            }
            opers.push(c);
        } else {
            // 지원하지 않는 연산자 입력시 에러처리 ex) 1 @ 2
            println!("올바르게 입력해주세요!");
            return;
        }
        if ch1 == ch2 {
            // 중위 표기법이 아닐시 에러처리
            println!("올바르게 입력해주세요!");
            return;
        }
    }

    if ch1 {
        // 중위 표기법이 아닐시 에러처리
        println!("올바르게 입력해주세요!");
        return;
    }

    loop {
        // opers의 남은 연산자들을 모두 postfix에 push
        match opers.pop() {
            Some(x) => postfix.push(Term::Operator(x)),
            _ => break,
        }
    }

    for c in postfix.iter() {
        if let Term::Operand(x) = c {
            result.push(*x);
        }
        let k;
        match c {
            Term::Operator('+') => {
                k = result.pop().unwrap() + result.pop().unwrap();
                result.push(k);
            },
            Term::Operator('-') => {
                k = - result.pop().unwrap() + result.pop().unwrap();
                result.push(k);
            },
            Term::Operator('*') => {
                k = result.pop().unwrap() * result.pop().unwrap();
                result.push(k);
            },
            Term::Operator('/') => {
                k = 1.0 / (result.pop().unwrap() / result.pop().unwrap());
                result.push(k);
            },
            _ => (),
        }
    }
    println!("결과: {}", result.pop().unwrap());
}