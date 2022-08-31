use crate::config;

pub(crate) struct Formatter
{
	pub(crate) verbose : bool,
	pub(crate) config : config::Config,
}

impl Formatter
{
	pub(crate) fn format(&self,content:String)
	{
		for line in content.lines()
		{
			println!("{}", line);
		}
	}
}