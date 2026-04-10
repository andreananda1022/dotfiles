local M = {}

local state = {
  buf = nil,
  win = nil,
  win_opts = nil,
  augroup = nil,
  laststatus = nil
}

local function create_buffer()
  local buf = vim.api.nvim_create_buf(false, true)

  vim.api.nvim_set_option_value("filetype", "dashboard", { buf = buf })
  vim.api.nvim_set_option_value("buftype", "nofile", { buf = buf })
  vim.api.nvim_set_option_value("bufhidden", "wipe", { buf = buf })
  vim.api.nvim_set_option_value("swapfile", false, { buf = buf })
  vim.api.nvim_set_option_value("modifiable", false, { buf = buf })

  return buf
end

local function center_lines(win, lines)
  local width = vim.api.nvim_win_get_width(win)
  local height = vim.api.nvim_win_get_height(win)

  local centered = {}
  local top_padding = math.floor((height - #lines) / 2)
  
  for _ = 1, math.max(top_padding, 0) do
    centered[#centered + 1] = ""
  end

  for _, line in ipairs(lines) do
    local line_width = vim.fn.strdisplaywidth(line)
    local padding = math.floor((width - line_width) / 2)
    centered[#centered + 1] = string.rep(" ", math.max(padding, 0)) .. line
  end

  return centered
end

local function get_content()
  return {
    [[                                                                       ]],
    [[                                                                     ]],
    [[       ████ ██████           █████      ██                     ]],
    [[      ███████████             █████                             ]],
    [[      █████████ ███████████████████ ███   ███████████   ]],
    [[     █████████  ███    █████████████ █████ ██████████████   ]],
    [[    █████████ ██████████ █████████ █████ █████ ████ █████   ]],
    [[  ███████████ ███    ███ █████████ █████ █████ ████ █████  ]],
    [[ ██████  █████████████████████ ████ █████ █████ ████ ██████ ]],
    [[                                                                       ]],
    "2026年、夜空の手によって生み出された。",
    "",
    "",
    "  New file      w",
    "",
    "  Explore       e",
    "",
    "  Quit          q",
    "",
    "",
    ""
  }
end

local function render()
  if not (state.buf and vim.api.nvim_buf_is_valid(state.buf)) then
    return
  end

  vim.api.nvim_set_option_value("modifiable", true, { buf = state.buf })

  local lines = center_lines(state.win, get_content())
  vim.api.nvim_buf_set_lines(state.buf, 0, -1, false, lines)

  vim.api.nvim_set_option_value("modifiable", false, { buf = state.buf })
end

local function cleanup()
  if state.win and vim.api.nvim_win_is_valid(state.win) then
    vim.api.nvim_set_option_value("number", state.win_opts.number, { win = state.win })
    vim.api.nvim_set_option_value("relativenumber", state.win_opts.relativenumber, { win = state.win })
  end

  if state.laststatus then
    vim.opt.laststatus = state.laststatus
  end
end

local function set_keymaps(buf)
  vim.keymap.set("n", "a", "<NOP>", { buffer = buf })
  vim.keymap.set("n", "i", "<NOP>", { buffer = buf })
  vim.keymap.set("n", "v", "<NOP>", { buffer = buf })
  vim.keymap.set("n", "w", ":enew<CR>", { buffer = buf, silent = true })
  vim.keymap.set("n", "e", ":Explore<CR>", { buffer = buf, silent = true })
  vim.keymap.set("n", "q", ":qa<CR>", { buffer = buf, silent = true })
end

function M.open()
  state.win = vim.api.nvim_get_current_win()
  state.buf = create_buffer()

  vim.api.nvim_set_current_buf(state.buf)

  state.win_opts = {
    number = vim.api.nvim_get_option_value("number", { win = state.win }),
    relativenumber = vim.api.nvim_get_option_value("relativenumber", { win = state.win })
  }

  vim.api.nvim_set_option_value("number", false, { win = state.win })
  vim.api.nvim_set_option_value("relativenumber", false, { win = state.win })

  state.laststatus = vim.opt.laststatus:get()
  vim.opt.laststatus = 0

  render()
  set_keymaps(state.buf)

  vim.api.nvim_create_autocmd({ "BufLeave", "BufWipeout" }, {
    group = state.augroup,
    buffer = state.buf,
    once = true,
    callback = cleanup
  })

  vim.api.nvim_create_autocmd("WinResized", {
    group = state.augroup,
    buffer = state.buf,
    callback = render
  })
end

function M.setup()
  if vim.fn.argc() > 0 then
    return
  end

  vim.opt.shortmess:append("I")

  state.augroup = vim.api.nvim_create_augroup("Dashboard", { clear = true })

  vim.api.nvim_create_autocmd("VimEnter", {
    group = state.augroup,
    once = true,
    callback = M.open
  })
end

return M
