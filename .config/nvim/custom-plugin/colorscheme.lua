local M = {}

function M.setup()
  local colors = {
    bg = "#1B211A",
    fg = "#DCD7BA",
    keyword = "#957FB8",
    string = "#76946A",
    comment = "#727169",
    func = "#7e9cd8",
    identifier = "#C0A36E"
  }

  local highlight = {
    Normal = { bg = colors.bg, fg = colors.fg },
    Comment = { fg = colors.comment, italic = true },
    String = { fg = colors.string },
    Keyword = { fg = colors.keyword, bold = true },
    Function = { fg = colors.func},
    Special = { fg = colors.identifier },
    Identifier = { fg = colors.identifier }
  }

  for group, opts in pairs(highlight) do
    vim.api.nvim_set_hl(0, group, opts)
  end
end

return M
