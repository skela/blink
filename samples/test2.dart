class Widget{
}

class SomeWidget extends Widget {
	final String title;
	final String message;

	SomeWidget({required this.title, required this.message});
}

class TestSuperClass { 

}

class TestClass extends TestSuperClass {
	int test = 0;

	void printHello() {
		print('hello');
	}

	void ifs(){
		if (test == 0) print("test is 0");
		else if (test == 1) print("test is 1");
		else print("test is something else");

		if (test == 0)
			print("test is 0");
		else if (test == 1)
			print("test is 1");
		else
			print("test is something else");

		if (test == 0) {
			print("test is 0");
		} 
		else if (test == 1) {
			print("test is 1");
		} 
		else {
			print("test is something else");
		}
	}

	Widget simpleWidget1() {
		return SomeWidget(
			title: "Test",
			message: "Message"
		);
	}

	Widget simpleWidget2(){
		return SomeWidget(
			title: "Test",
			message: "Message",
		);
	}
}
