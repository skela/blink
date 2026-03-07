// Trailing comma formatting test cases

// Case 1: trailing comma present — must expand to multi-line
void caseTrailingCommaExpand()
{
	myFunction(
		arg1,
		arg2,
		arg3,
	);
}

// Case 2: no trailing comma, fits on one line — must collapse to single line
void caseNoTrailingCommaCollapse()
{
	myFunction(arg1, arg2, arg3);
}

// Case 3: no trailing comma, does NOT fit on one line — must expand without adding comma
void caseNoTrailingCommaTooLong()
{
	myFunction(
		veryLongArgumentNameOne,
		veryLongArgumentNameTwo,
		veryLongArgumentNameThree
	);
}

// Case 4: already correctly formatted multi-line with trailing comma — no change
void caseAlreadyMultiLine()
{
	myFunction(
		arg1,
		arg2,
		arg3,
	);
}

// Case 5: already correctly formatted single-line — no change
void caseAlreadySingleLine()
{
	myFunction(arg1, arg2, arg3);
}

// Case 6: empty list — always single line, no change
void caseEmptyList()
{
	myFunction();
}

// Case 7: single argument with trailing comma — must expand
void caseSingleArgWithTrailingComma()
{
	myFunction(
		singleArg,
	);
}

// Case 8: named parameters with trailing comma
void caseNamedParams()
{
	myFunction(
		positional,
		named: value,
	);
}

// Case 9: list literal with trailing comma
void caseListLiteral()
{
	var list = [
		item1,
		item2,
		item3,
	];
}

// Case 10: list literal without trailing comma, fits — collapse
void caseListNoTrailingComma()
{
	var list = [item1, item2];
}

// Case 11: nested structures, both with trailing commas
void caseNested()
{
	OuterWidget(
		child: InnerWidget(
			param1,
			param2,
		),
		color: Colors.red,
	);
}

// Case 12: inline comment forces multi-line even without trailing comma
void caseInlineComment()
{
	myFunction(
		arg1,
		// important
		arg2
	);
}

// Case 13: constructor call with trailing comma
void caseConstructor()
{
	return MyClass(
		field1: "hello",
		field2: 42,
	);
}
