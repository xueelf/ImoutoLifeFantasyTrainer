local config = require('config')
local keymap = require('keymap')
local cheat = require('cheat')
local cover = require('views.cover')

local header = {
    { title = '状态', span = 45 },
    { title = '热键', span = 45 },
    { title = '功能', span = 140 },
    { title = '描述', span = 170 },
}
local status = {
    enable = '●',
    disable = '○',
}
local width, height = config.window.width - cover.width, config.window.height

local pool = {}

local function addShortcut(owner, code, action)
    if not owner then
        return
    end
    local callback = function()
        local method = cheat[action]

        if not method then
            showMessage(string.format('Action "%s" is not defined', action))
        else
            local enabled = owner.Caption == status.disable
            local success, result = pcall(method, enabled)

            if success then
                owner.Caption = enabled and status.enable or status.disable
            else
                showMessage(result)
            end
        end
    end
    createHotkey(callback, code)
    table.insert(pool, callback)
end

local function emit(index)
    pool[index]()
end

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

        return Item
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
        emit(sender.ItemIndex + 1)

        sender.Selected = nil
        sender.ItemIndex = -1
    end

    for _, value in ipairs(header) do
        addCol(value.title, value.span)
    end

    for _, value in pairs(keymap) do
        local Item = addRow(value.hotkey, value.effect, value.description)

        addShortcut(Item, value.code, value.action)
    end

    return Feature
end

return {
    create = create,
}
