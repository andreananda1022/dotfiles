local M = {}

local DICTIONARY = { 'println!("");', "String", 'String::from("");', "return", "while" }

local state = {
  win = nil,
  buf = nil,
}

local function close_window()
  if state.win and vim.api.nvim_win_is_valid(state.win) then
    vim.api.nvim_win_close(state.win, true)
  end
  state.win = nil
  state.buf = nil
end

local function get_base_word()
  local line = vim.api.nvim_get_current_line()
  local col = vim.api.nvim_win_get_cursor(0)[2]
  local base_word = line:sub(1, col):match("[%w_]+$")
  return base_word or ""
end

local function find_matches(base_word)
  local matches = {}
  for _, word in ipairs(DICTIONARY) do
    if word:sub(1, #base_word) == base_word then
      table.insert(matches, word)
    end
  end
  return matches
end

local function create_float_window(matches)
  local buf = vim.api.nvim_create_buf(false, true)
  vim.api.nvim_buf_set_lines(buf, 0, -1, false, matches)

  vim.api.nvim_set_option_value("bufhidden", "wipe", { buf = buf })

  local max_width = 0
  for _, text in ipairs(matches) do
    local width = vim.api.nvim_strwidth(text)
    if width > max_width then max_width = width end
  end

  local ui = vim.api.nvim_list_uis()[1]
  local limit_width = ui and ui.width or 80
  local limit_height = ui and ui.height or 20

  local width = math.min(max_width + 2, limit_width - 10)
  local height = math.min(#matches, limit_height - 5)

  local win = vim.api.nvim_open_win(buf, true, {
    relative = "cursor",
    row = 1,
    col = 0,
    width = width,
    height = height,
    style = "minimal",
    border = "none"
  })

  vim.api.nvim_set_option_value("winhl", "Normal:Pmenu,FloatBorder:PmenuBorder,CursorLine:PmenuSel", { win = win })
  vim.api.nvim_set_option_value("cursorline", true, { win = win })

  return win, buf
end

function M.trigger_completion()
  if state.win and vim.api.nvim_win_is_valid(state.win) then
    return
  end

  local base_word = get_base_word()
  if base_word == "" then return end

  local matches = find_matches(base_word)
  if #matches == 0 then return end

  local orig_win = vim.api.nvim_get_current_win()
  local orig_buf = vim.api.nvim_get_current_buf()
  local cursor_pos = vim.api.nvim_win_get_cursor(orig_win)
  local row, col = cursor_pos[1], cursor_pos[2]

  state.win, state.buf = create_float_window(matches)
  vim.cmd("stopinsert")

  vim.api.nvim_create_autocmd("WinLeave", {
    buffer = state.buf,
    once = true,
    callback = close_window,
  })

  local keymap_opts = { buffer = state.buf, silent = true, noremap = true }
  vim.keymap.set("n", "<Esc>", close_window, keymap_opts)
  vim.keymap.set("n", "q", close_window, keymap_opts)

  vim.keymap.set("n", "<CR>", function()
    local selected_text = vim.api.nvim_get_current_line()
    close_window()

    vim.api.nvim_set_current_win(orig_win)

    local line_text = vim.api.nvim_buf_get_lines(orig_buf, row - 1, row, false)[1]
    local line_to_cursor = line_text:sub(1, col)
    local word_start = line_to_cursor:find("[%w_]*$")
    local start_col = (word_start or (col + 1)) - 1

    vim.api.nvim_buf_set_text(orig_buf, row - 1, start_col, row - 1, col, { selected_text })

    local new_col = start_col + #selected_text
    vim.api.nvim_win_set_cursor(orig_win, { row, new_col })

    vim.cmd("startinsert")
    if new_col >= #vim.api.nvim_buf_get_lines(orig_buf, row - 1, row, false)[1] then
      vim.api.nvim_feedkeys(vim.api.nvim_replace_termcodes("<Right>", true, false, true), "n", true)
    end
  end, keymap_opts)
end

function M.setup()
  vim.keymap.set("i", "<C-Space>", M.trigger_completion, { desc = "Trigger Custom Completion" })
end

return M
