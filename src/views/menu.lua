local config = require("config")

local DOCUMENTATION_URL = "https://blog.yuki.sh/posts/05064242ec86/"
local ABOUT_TEXT = string.format([[Version %s

Powered by Cheat Engine
Copyright © 2025 by Yuki<admin@yuki.sh>]], config.version)

local function create(owner)
    local Menu = createMainMenu(owner)

    local MenuSeparator = createMenuItem(MenuHelp)
    MenuSeparator.Caption = "-"

    local MenuHelp = createMenuItem(Menu)
    MenuHelp.Caption = "帮助(&H)"

    local MenuDocumentation = createMenuItem(MenuHelp)
    MenuDocumentation.Caption = "文档(&D)"
    MenuDocumentation.OnClick = function()
        shellExecute(DOCUMENTATION_URL)
    end

    local MenuAbout = createMenuItem(MenuHelp)
    MenuAbout.Caption = "关于(&A)"
    MenuAbout.OnClick = function()
        showMessage(ABOUT_TEXT)
    end

    MenuHelp.add(MenuDocumentation)
    MenuHelp.add(MenuSeparator)
    MenuHelp.add(MenuAbout)
    Menu.Items.add(MenuHelp)
end

return {
    create = create,
}
