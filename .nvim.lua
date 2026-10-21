-- You must enable the exrc setting in neovim for this config file to be used.
local rust_analyzer = {
    cargo = {
        target = "xtensa-esp32-none-elf",
        allTargets = false,
        extraEnv = { RUSTUP_TOOLCHAIN = "esp" },
    },
}

-- Note the rustaceanvim name of the language server is rust-analyzer with a dash.
vim.lsp.config("rust-analyzer", {
	cmd = { "rustup", "run", "stable", "rust-analyzer" },
    settings = {
        ["rust-analyzer"] = rust_analyzer,
    },
})
