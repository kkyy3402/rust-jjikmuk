fn main() {
    // 헬로월드 찍기
    println!("Hello, world!");

    // 변수 선언 (i32 : integer 32 bit가 기본타입이라고 함)
    let var = 1;
    println!("{}이 출력되어부렀어요",var);

    let mut var2 = 1.0;
    println!("{}이 출력되어부렀어요",var);

    //이렇게 하면 메모리가 힙에 잡힌답니다. 그리고 변경가능하담니다. 
    let mut s = String::from("STRING EXAMPLE");
    println!("string example : {}", s);

    //이렇게 하면 스택에 잡히나? 위에서 스코프 안에서 사용가능여부가 달라지겠져? 이걸 뭐 소유권 어쩌구 하는데, 
    //정확히는 모르것습니다. 이건 찍먹이니 패스
    let mut s2 = "뭔 차이세요??";
    println!("string example2 : {}", s2);

    var2 = 3.0;
    println!("var2 changed {}",var2);

    //함수
    fn func_sample(){
        println!("Function is called!");
    }

    //매개변수가 들어가는 함수. 타입이 선언되어야 함
    func_sample();

    fn add(x: i32, y: i32){
        println!("result : {}",x+y);
    }

    add(1,2);

    // if문
    let _number = 3;
    if _number > 5 {
        println!("5 초과");
    }else {
        println!("5 이하");
    }

    //for loop like c style
    for i in 0..10 {
        println!("{}", i);
    }

    let itemsForLoop = ["아 이걸 지금봤네", "아 나이따"];

    for item in itemsForLoop {
        println!("{}", item);
    }


}

