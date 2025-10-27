local config = require("config")

local function create()
    local Window = createForm(false)

    Window.Caption = config.window.title
    Window.Width, Window.Height = config.window.width, config.window.height
    Window.OnClose = closeCE

    return Window
end

return {
    create = create,
}

-- local function addRow(hotkey, effect, description)
--     local Item = List.Items.add()

--     Item.Caption = ''
--     Item.SubItems.add(hotkey)
--     Item.SubItems.add(effect)
--     Item.SubItems.add(description)

--     return Item
-- end

-- F1 = addRow("F1", "三倍速", "游戏变速")
-- F2 = addRow("F2", "见面 5 秒结束战斗", "战斗阶段直接获得胜利")
