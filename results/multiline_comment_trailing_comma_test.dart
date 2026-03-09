// Test case for multi-line element with trailing comma and inline comment
// Issue: formatter was removing trailing comma when comment follows it

void testCase()
{
	var list = [
		item1,
		if (condition)
			cell("Is this working?", model.isThisWorking), // TODO: Localise this
		item2,
	];
}
