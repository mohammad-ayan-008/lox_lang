fun test(){
  print (a);
}

var a = 2;
//test();
{
  var a = "local";
  test();
}
{
  test();
}

fun maketest(){
  var a = 2;
  fun test(){
    print(a);
  }
  return test;
}

var tester = maketest();
tester();
{
  var a = "local";
  tester();
}

fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print (i);
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2"


fun thrice(fn) {
  for (var i = 1; i <= 3; i = i + 1) {
    fn(i);
  }
}

thrice( -> (a){
  print (a);
});
// "1".
// "2".
// "3".

var sum  = -> (a,b){
  print("The Sum of a + b is "+a+b);
};

sum(9,0);
