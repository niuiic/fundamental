local core = require("core")

local overseer = require("overseer")

local root_path = core.file.root_path()
local script_path = root_path .. "/.nvim/task/task.sh"

local collect_args = function(args, is_bin, custom_args)
	local pkg = string.match(vim.api.nvim_buf_get_name(0), string.gsub(root_path, "%-", "%%-") .. "/([^/]+)")
	if pkg ~= nil then
		if is_bin then
			args = core.lua.list.merge(args, { "--bin", pkg })
		else
			args = core.lua.list.merge(args, { "--package", pkg })
		end
	end

	if custom_args == true then
		vim.ui.input({ prompt = "Args: " }, function(input)
			if input ~= nil then
				args = core.lua.list.merge(args, { "--" })
				args = core.lua.list.merge(args, core.lua.string.split(input, " "))
			end
		end)
	end

	return args
end

overseer.register_template({
	name = "check",
	builder = function()
		return {
			cmd = { script_path },
			args = { "check" },
			components = { "on_exit_set_status", "on_output_quickfix", "on_complete_notify" },
		}
	end,
})

overseer.register_template({
	name = "run",
	builder = function()
		local args = { "run" }
		return {
			cmd = { "cargo" },
			args = collect_args(args, true, true),
			components = { "on_exit_set_status", "on_complete_notify" },
		}
	end,
})

overseer.register_template({
	name = "miri",
	builder = function()
		local args = { "+nightly", "miri", "run" }
		return {
			cmd = { "cargo" },
			args = collect_args(args, true, true),
			components = { "on_exit_set_status", "on_output_quickfix", "on_complete_notify" },
		}
	end,
})

overseer.register_template({
	name = "miri test",
	builder = function()
		local args = { "+nightly", "miri", "test" }
		return {
			cmd = { "cargo" },
			args = collect_args(args),
			components = { "on_exit_set_status", "on_output_quickfix", "on_complete_notify" },
		}
	end,
})
