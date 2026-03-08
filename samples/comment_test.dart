// Comment test cases — commented out code must never be touched by the formatter

// Case 1: single-line comment containing a trailing comma call — must not expand
// myFunction(arg1, arg2,)

// Case 2: doc comment containing a trailing comma call — must not expand
/// myFunction(arg1, arg2,)

// Case 3: inline block comment on one line — must not expand
/* myFunction(arg1, arg2,) */

// Case 4: multi-line block comment containing a trailing comma call — must not expand
/*
Widget build(BuildContext context) {
  return Scaffold(
    appBar: appBar,
    body: body,
    backgroundColor: style.backgroundColor,
  );
}
*/

// Case 5: real code after comments — trailing comma must still expand normally
void realCode() {
  myFunction(arg1, arg2,);
}
