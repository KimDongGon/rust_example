// rust의 변수는 기본적으로 불변 -> 가변은 mut 사용
// println!에 있는 {}는 변경자
// rand는 crate의 일종, library crate이며, crate란 패키지를 의미
// cargo.toml에 crate와 버전을 명시하는 Semantic Versioning을 입력함으로써 가져오는 버전명이 달라도 SemVer 덕에 코드 호환
// 이러한 외부 의존성 추가를 통해 Cargo가 Crates.io 데이터의 복사본인 레지스트리에서 명시한 rand crate에 해당하는 데이터를 가져옴
// 또한 rand가 의존하는 libc도 동시에 가져옴
// cargo를 통해 build하면 cargo.lock 파일을 생성하며, cargo.lock은 crate 업데이트로 인한 build crash를 막아준다.
// cargo update를 통해서 cargo.toml에 입력한 버전으로 update
// cargo doc --open 명령어를 통해 의존 패키지들의 문서를 build 해서 브라우저에서 볼 수 있음
// std::cmp는 비교할 때 사용
// parse() 메소드를 통해서 파싱할 수 있으며, 타입을 명시해줘야 함.
// rust는 이전에 선언했던 변수를 다시 선언해서 사용할 수 있으며, Shadowing이라고 함. (다른 타입으로 바꾸고 싶을 때 사용)

// 외부에 의존하는 rand crate가 있음을 알림
extern crate rand;

// 스코프에 prelude(전..?) 타입이 없으면 use를 통해서 가져와야함
use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
