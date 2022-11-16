struct Nil {}
struct Cons<Head, Tail>(Head, Tail);

struct N0 {}
struct N1 {}
struct N2 {}
struct N3 {}
struct N4 {}
struct N5 {}
struct N6 {}
struct N7 {}
struct N8 {}
struct N9 {}
struct N10 {}
struct N11 {}
struct N12 {}
struct N13 {}
struct N14 {}

trait Succ {
    type Out;
}

impl<Tail> Succ for Cons<N0, Tail> {
    type Out = Cons<N1, Tail>;
}
impl<Tail> Succ for Cons<N1, Tail> {
    type Out = Cons<N2, Tail>;
}
impl<Tail> Succ for Cons<N2, Tail> {
    type Out = Cons<N3, Tail>;
}
impl<Tail> Succ for Cons<N3, Tail> {
    type Out = Cons<N4, Tail>;
}
impl<Tail> Succ for Cons<N4, Tail> {
    type Out = Cons<N5, Tail>;
}
impl<Tail> Succ for Cons<N5, Tail> {
    type Out = Cons<N6, Tail>;
}
impl<Tail> Succ for Cons<N6, Tail> {
    type Out = Cons<N7, Tail>;
}
impl<Tail> Succ for Cons<N7, Tail> {
    type Out = Cons<N8, Tail>;
}
impl<Tail> Succ for Cons<N8, Tail> {
    type Out = Cons<N9, Tail>;
}
impl<Tail> Succ for Cons<N9, Tail> {
    type Out = Cons<N10, Tail>;
}
impl<Tail> Succ for Cons<N10, Tail> {
    type Out = Cons<N11, Tail>;
}
impl<Tail> Succ for Cons<N11, Tail> {
    type Out = Cons<N12, Tail>;
}
impl<Tail> Succ for Cons<N12, Tail> {
    type Out = Cons<N13, Tail>;
}
impl<Tail> Succ for Cons<N13, Tail> {
    type Out = Cons<N14, Tail>;
}
impl<Tail> Succ for Cons<N14, Nil> {
    type Out = Cons<N0, Cons<N1, Nil>>;
}

struct Fizz {}
struct Buzz {}
struct FizzBuzz {}

trait FizzBuzzable {
    type Out;
}

impl<Tail> FizzBuzzable for Cons<N0, Tail> {
    type Out = FizzBuzz;
}
impl<Tail> FizzBuzzable for Cons<N3, Tail> {
    type Out = Fizz;
}
impl<Tail> FizzBuzzable for Cons<N5, Tail> {
    type Out = Buzz;
}
impl<Tail> FizzBuzzable for Cons<N6, Tail> {
    type Out = Fizz;
}
impl<Tail> FizzBuzzable for Cons<N9, Tail> {
    type Out = Fizz;
}
impl<Tail> FizzBuzzable for Cons<N10, Tail> {
    type Out = Buzz;
}
impl<Tail> FizzBuzzable for Cons<N12, Tail> {
    type Out = Fizz;
}
impl<Tail> FizzBuzzable for Cons<N1, Tail> {
    type Out = Cons<N1, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N2, Tail> {
    type Out = Cons<N2, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N4, Tail> {
    type Out = Cons<N4, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N7, Tail> {
    type Out = Cons<N7, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N8, Tail> {
    type Out = Cons<N8, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N11, Tail> {
    type Out = Cons<N11, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N13, Tail> {
    type Out = Cons<N13, Tail>;
}
impl<Tail> FizzBuzzable for Cons<N14, Tail> {
    type Out = Cons<N14, Tail>;
}

type Inc<N> = <N as Succ>::Out;

type Modulo<N> = <N as FizzBuzzable>::Out;

trait Range {
    type Out;
}

impl<RTail> Range for Cons<Cons<N0, Nil>, RTail> {
    type Out = RTail;
}

impl<NHead, NTail, RTail> Range for Cons<Inc<Cons<NHead, NTail>>, RTail> {
    type Out =
        Cons<Modulo<Inc<Cons<NHead, NTail>>>, FizzBuzzRange<Cons<Cons<NHead, NTail>, RTail>>>;
}

type FizzBuzzRange<N> = <N as Range>::Out;

fn main() {
    type Test = FizzBuzzRange<Cons<Cons<N8, Cons<N1, Nil>>, Nil>>;
    let dummy: Test;
}
