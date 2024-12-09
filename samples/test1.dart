import 'dart:convert';

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
	CatRace cat = CatRace.Meow2;
	DogRace dog = DogRace.Woof3;
	GerbilRace gerbil = GerbilRace.Gerb1;

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

	String whatever3 = '''
	
		{
			"somejson":"testing",
			"yay":1
		}
	
	''';

	void printHello() 
	{
		print('hello');
	}

	String quotes()
	{
		Encoding.getByName("utf-8");
		String str = 'potato+';
		String output = str.replaceAll('-', '+').replaceAll('_', '/');
		return output;
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

			} break;
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

	void switchWithinSwitch()
	{
		switch(animal){
			case Animal.Cat: 
				switch(cat){
					case CatRace.Meow1: 
					break;
					case CatRace.Meow2:
					break;
					case CatRace.Meow3:
					break;
				}
			break;
			case Animal.Dog: 
				switch(dog){
					case DogRace.Woof1: break;
					case DogRace.Woof2: break;
					case DogRace.Woof3: break;
				}
			break;
			case Animal.Gerbil: 
				switch(gerbil){
					case GerbilRace.Gerb1: 
						break;
					case GerbilRace.Gerb2: 
						break;
					case GerbilRace.Gerb3:
					{
						}
						break;
				}
			break;
		}
	}

	void breakInForLoopWithinSwitch()
	{
		switch(animal)
		{
			case Animal.Cat:
				for (var i = 0; i<5; i++)
					if (i == 2)
						break; // TODO: This is incorrecly adjusted when it shouldnt be
			break;
			case Animal.Dog:
			break;
			case Animal.Gerbil:
			break;
		}
	}

	Future<int> commentedOut() async
	{
		// var comment = await getComment();
		// var res = await check(comment);
		// if (checkResult(res))
		// {
		// 	await storeComment(comment);
		// 	return res;
		// }
		// return res;
		return Future.value(1);
	}
}

enum Animal
{
	Cat,
	Dog,	
	Gerbil,
}

enum CatRace
{
	Meow1,
	Meow2,
	Meow3,
}

enum DogRace
{
	Woof1,
	Woof2,
	Woof3,
}

enum GerbilRace
{
	Gerb1,
	Gerb2,
	Gerb3,
}
