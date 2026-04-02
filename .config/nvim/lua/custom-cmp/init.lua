local M = {}

local DICTIONARY = {
  { word = "function", kind = "keyword" },
  { word = "function", kind = "keyword" },
  { word = "local", kind = "keyword" },
  { word = "return", kind = "keyword" },
  { word = "while", kind = "keyword" }
}

local state = {
  float_win = nil,
  float_buf = nil,
  orig_win = nil,
  orig_buf = nil,
  row = nil,
  col_start = nil,
  matches = {},
}

local function is_window_valid(win)
  return win and vim.api.nvim_win_is_valid(win)
end

local function reset_state()
  state.float_win = nil
  state.float_buf = nil
  state.matches = {}
end

function M.close_window()
  if is_window_valid(state.float_win) then
    vim.api.nvim_win_close(state.float_win, true)
  end
  reset_state()
end

local function create_float(lines)
  local buf = vim.api.nvim_create_buf(false, true)

  vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)
  vim.api.nvim_buf_set_option(buf, "modifiable", false)

  local win = vim.api.nvim_open_win(buf, true, {
    relative = "cursor",
    row = 1,
    col = 0,
    width = 28,
    height = #lines,
    style = "minimal",
    border = "solid"
  })

  vim.api.nvim_win_set_option(win, "winhighlight", "Normal:NormalFloat,FloatBorder:FloatBorder")
  vim.api.nvim_win_set_option(win, "cursorline", true)

  return win, buf
end

local function get_base_word()
  local line = vim.api.nvim_get_current_line()
  local cursor = vim.api.nvim_win_get_cursor(0)

  local row = cursor[1] - 1
  local col = cursor[2]

  local start = col
  while start > 0 and line:sub(start, start):match("[%w_]") do
    start = start - 1
  end

  return {
    row = row,
    col = col,
    start = start,
    base = line:sub(start + 1, col)
  }
end

local function find_matches(base)
  local result = {}
  for _, item in ipairs(DICTIONARY) do
    if item.word:sub(1, #base) == base then
      table.insert(result, item)
    end
  end
  return result
end

local function render_lines(matches)
  local lines = {}
  for _, item in ipairs(matches) do
    table.insert(lines, string.format("%s", item.word))
  end
  return lines
end

function M.trigger_completion()
  if state.float_win then
    return
  end

  local base_info = get_base_word()
  if base_info.base == "" then
    return
  end

  local matches = find_matches(base_info.base)
  if #matches == 0 then
    return
  end

  state.matches = matches
  state.orig_win = vim.api.nvim_get_current_win()
  state.orig_buf = vim.api.nvim_get_current_buf()
  state.row = base_info.row
  state.col_start = base_info.start

  local lines = render_lines(matches)
  state.float_win, state.float_buf = create_float(lines)

  vim.keymap.set("n", "<CR>", function()
    local idx = vim.api.nvim_win_get_cursor(state.float_win)[1]
    local item = state.matches[idx]

    vim.api.nvim_buf_set_text(
      state.orig_buf,
      state.row,
      state.col_start,
      state.row,
      base_info.col,
      { item.word }
    )

    vim.api.nvim_set_current_win(state.orig_win)
    vim.api.nvim_win_set_cursor(
      state.orig_win,
      { state.row + 1, state.col_start + #item.word }
    )
    vim.cmd("startinsert")
  end, { buffer = state.float_buf })

  vim.keymap.set("n", "<Esc>", function()
    M.close_window()
    vim.api.nvim_set_current_win(state.orig_win)
    vim.cmd("startinsert")
  end, { buffer = state.float_buf })

  vim.api.nvim_create_autocmd(
    { "CursorMoved", "BufLeave" },
    { once = true, callback = M.close_window }
  )

    M.close_window()
end

function M.setup()
  vim.keymap.set("i", "<C-Space>", function()
    M.trigger_completion()
  end, { desc = "mycmp completion" })
end

return M
