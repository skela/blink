class Widget
{

}

class SomeWidget extends Widget 
{
	final String title;
	final String message;

	SomeWidget({required this.title, required this.message});
}

class TestSuperClass 
{

}

class TestClass extends TestSuperClass 
{
	int test = 0;
	Animal animal = Animal.Gerbil;

	String whatever1 = 
	"""asd
    hi i want spaces and curlies {
		because im a string literal,
	 who would prefer to remain exactly the way i am
	asd""";

	String whatever2 = """asd
    hi i want spaces and curlies {
		because im a string literal,
	 who would prefer to remain exactly the way i am
	asd""";

	void printHello() 
	{
		print('hello');
	}

	void ifs() 
	{
		if (test == 0) print("test is 0");
		else if (test == 1) print("test is 1");
		else print("test is something else");

		if (test == 0)
			print("test is 0");
		else if (test == 1)
			print("test is 1");
		else
			print("test is something else");

		if (test == 0) 
		{
			print("test is 0");
		} else if (test == 1) 
		{
			print("test is 1");
		} else 
		{
			print("test is something else");
		}

	}


	Widget simpleWidget1() 
	{
		return SomeWidget(
			title: "Test",
			message: "Message"
		);
	}



	Widget simpleWidget2() 
	{
		return SomeWidget(
			title: "Test",
			message: "Message",
		);
	}

	void switches1()
	{
		switch(animal){
			case Animal.Cat: break;
			case Animal.Dog: break;
			case Animal.Gerbil: break;
		}

		switch(animal){
			case Animal.Cat: 
			break;
			case Animal.Dog: 
			break;
			case Animal.Gerbil: 
			break;
		}

		switch(animal){
			case Animal.Cat: 
				break;
			case Animal.Dog: 
				break;
			case Animal.Gerbil: 
				break;
		}
	}

	void switches2()
	{
		switch(animal){
			case Animal.Cat: {} break;
			case Animal.Dog: {} break;
			case Animal.Gerbil: {} break;
		}

		switch(animal){
			case Animal.Cat: {

			}
			break;
			case Animal.Dog: {

			}
			break;
			case Animal.Gerbil: {

			}
			break;
		}

		switch(animal){
			case Animal.Cat: {

			}
				break;
			case Animal.Dog: {

			}
				break;
			case Animal.Gerbil: {

			}
				break;
		}
	}
}

enum Animal
{
	Cat,
	Dog,	
	Gerbil,
}