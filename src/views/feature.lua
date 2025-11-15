local config = require("config")
local keymap = require("keymap")
local cover = require("views.cover")

local status = {
    enable = "●",
    disable = "○",
}
local header = {
    { title = "状态", span = 45 },
    { title = "热键", span = 45 },
    { title = "功能", span = 140 },
    { title = "描述", span = 170 },
}
local width, height = config.window.width - cover.width, config.window.height

local function create(owner)
    local Feature = createListView(owner)

    local function addCol(title, span)
        local Column = Feature.Columns.add()

        if span then
            Column.Width = span
        end
        Column.Caption = title
    end

    local function addRow(...)
        local args = { ... }
        local Item = Feature.Items.add()

        Item.Caption = status.disable
        Item.SubItems.text = table.concat(args, '\n')
    end

    Feature.Width, Feature.Height = width, height
    Feature.Left = cover.width
    Feature.BorderStyle = bsNone
    Feature.ReadOnly = true
    Feature.RowSelect = true
    Feature.ViewStyle = vsReport
    Feature.OnClick = function(sender)
        local Item = sender.Selected

        if not Item then
            return
        end
        local enabled = Item.Caption == status.enable

        if enabled then
            Item.Caption = status.disable
        else
            Item.Caption = status.enable
        end
        sender.Selected = nil
        sender.ItemIndex = -1
    end

    for _, value in ipairs(header) do
        addCol(value.title, value.span)
    end

    for _, value in pairs(keymap) do
        addRow(value.hotkey, value.effect, value.description)
    end

    return Feature
end

return {
    create = create,
}
