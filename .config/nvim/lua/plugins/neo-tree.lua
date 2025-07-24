return {
  "nvim-neo-tree/neo-tree.nvim",
  branch = "v3.x",
  lazy = false,
  dependencies = {
    "nvim-lua/plenary.nvim",
    "nvim-tree/nvim-web-devicons",
    "MunifTanjim/nui.nvim"
  },
  config = function()
    require("neo-tree").setup({
      window = {
        position = "left",
        width = 24
      }
    })
    vim.keymap.set("n", "<C-b>", "<Cmd>Neotree toggle<CR>")
  end
}
