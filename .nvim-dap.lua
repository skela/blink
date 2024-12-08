local dap = require("dap")

dap.adapters.lldb = {
	type = "executable",
	command = "/usr/bin/lldb-vscode",
	name = "lldb",
}

dap.configurations.rust = {
	{
		name = "analyze",
		type = "lldb",
		request = "launch",
		program = function()
			vim.fn.jobstart("cargo build")
			return vim.fn.getcwd() .. "/target/debug/blink"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		args = { "-a", "test" },
	},
	{
		name = "format",
		type = "lldb",
		request = "launch",
		program = function()
			vim.fn.jobstart("cargo build")
			return vim.fn.getcwd() .. "/target/debug/blink"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		args = { "-t", "test" },
	},
}
